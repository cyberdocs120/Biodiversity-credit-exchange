# Polygon Specification

## Overview

Habitat polygons define the geographic boundaries of biodiversity projects. Each polygon is identified by a 32-byte ID and has both on-chain and off-chain representations.

## On-Chain vs Off-Chain Data

### On-Chain (Stored in Contract)

The `mrv-oracle` contract stores a compact `HabitatPolygon` struct containing only essential fields:

| Field                | Type         | Description                           |
|----------------------|--------------|---------------------------------------|
| `polygon_id`         | `BytesN<32>` | Unique 32-byte identifier             |
| `geometry_ipfs_cid`  | `Bytes`      | IPFS CID of full GeoJSON geometry     |
| `bounding_box`       | `BoundingBox`| Min/max lat/lon (see below)           |
| `area_ha`            | `u64`        | Area in hectares                      |
| `biome`              | `u32`        | Biome classification (see below)      |
| `country`            | `BytesN<2>`  | ISO 3166-1 alpha-2 country code       |
| `project_id`         | `BytesN<32>` | Associated project identifier         |
| `registered_at`      | `u64`        | Registration timestamp                |
| `active`             | `bool`       | Whether polygon is currently active   |
| `total_credits_minted` | `u64`     | Running total of credits minted       |
| `total_credits_retired` | `u64`    | Running total of credits retired      |

### Off-Chain (IPFS)

Full geometry is stored as GeoJSON on IPFS, referenced by the `geometry_ipfs_cid` field. The GeoJSON schema:

```json
{
  "type": "Feature",
  "geometry": {
    "type": "MultiPolygon",
    "coordinates": [[[
      [lon, lat],
      [lon, lat],
      ...
    ]]]
  },
  "properties": {
    "polygon_id": "0x...",
    "name": "Amazon Reserve Plot A",
    "project_id": "0x...",
    "area_ha": 4500,
    "biome": "TropicalForest",
    "country": "BR",
    "methodology": "BDCX-TF-v1.0",
    "registered_at": 1718000000,
    "registration_authority": "0x..."
  }
}
```

## Coordinate Encoding

All on-chain coordinates use **fixed-point i64** with a multiplier of **1,000,000**:

```
on_chain_value = round(real_coordinate × 1,000,000)

Example:
  Latitude:  -23.550520°  →  -23_550_520
  Longitude: -46.633309°  →  -46_633_309
```

This gives sub-meter precision (~0.11 m at the equator).

### BoundingBox Struct

```rust
pub struct BoundingBox {
    pub min_lat: i64,   // min latitude  × 1,000,000
    pub max_lat: i64,   // max latitude  × 1,000,000
    pub min_lon: i64,   // min longitude × 1,000,000
    pub max_lon: i64,   // max longitude × 1,000,000
}
```

## Point-in-Polygon (Ray-Casting Algorithm)

The `retirement` contract implements point-in-polygon using the ray-casting algorithm for on-chain verification of containment claims.

### Pseudocode

```
function point_in_polygon(point, polygon):
    inside = false
    n = length(polygon)
    j = n - 1
    
    for i in 0 to n-1:
        yi = polygon[i].lat
        yj = polygon[j].lat
        xi = polygon[i].lon
        xj = polygon[j].lon
        
        if ((yi > point.lat) != (yj > point.lat)):
            intersect_x = (xj - xi) * (point.lat - yi) / (yj - yi) + xi
            if point.lon < intersect_x:
                inside = not inside
        
        j = i
    
    return inside
```

### Rust Implementation

```rust
pub fn point_in_polygon(
    point: (i64, i64),
    polygon: Vec<(i64, i64)>,
) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;
    for i in 0..n {
        if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
            && (point.0 < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
                / (polygon[j].1 - polygon[i].1) + polygon[i].0)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}
```

### Limitations

- Assumes polygon is **simple** (non-self-intersecting)
- Does not handle holes (interior rings)
- Optimized for MVP; production should use winding number algorithm for robustness
- Vertex count is unbounded; gas costs scale linearly with polygon complexity

## Polygon Lifecycle

```
         ┌──────────────────────────────────┐
         │         Registered                │
         │  (register_polygon, admin)        │
         └────────────┬─────────────────────┘
                      │
                      ▼
         ┌──────────────────────────────────┐
         │           Active                  │
         │  - Surveys can be submitted       │
         │  - Credits can be minted          │
         └────────────┬─────────────────────┘
                      │
              ┌───────┴───────┐
              │               │
              ▼               ▼
   ┌────────────────┐  ┌────────────────┐
   │    Closed      │  │  Disputed      │
   │ (close_polygon)│  │ (via survey    │
   │  No new minting│  │  dispute flag) │
   └────────────────┘  └────────────────┘
```

| Operation            | Trigger          | Description                            |
|----------------------|------------------|----------------------------------------|
| `register_polygon`   | Admin            | Creates polygon in Active state        |
| `close_polygon`      | Admin            | Sets active=false, no new minting      |
| `dispute` (via survey)| Oracle          | Marks polygon as disputed              |
| `resolve_dispute`    | Admin            | Resolves dispute, may re-activate      |

## Biome Classification Scheme

| Code | Biome             | Description                        |
|------|-------------------|------------------------------------|
| 0    | TropicalForest    | Tropical rainforest, monsoon forest |
| 1    | TemperateForest   | Deciduous, coniferous, mixed forest|
| 2    | Grassland         | Savanna, prairie, steppe            |
| 3    | Wetland           | Peatland, marsh, swamp              |
| 4    | Mangrove          | Coastal mangrove forest             |
| 5    | CoralReef         | Coral reef ecosystems               |
| 6    | Other             | Desert, tundra, urban, etc.        |

## Minimum Area Requirements

| Biome             | Min Area (ha) | Max Area (ha) |
|-------------------|---------------|---------------|
| TropicalForest    | 1             | 100,000       |
| TemperateForest   | 1             | 100,000       |
| Grassland         | 5             | 100,000       |
| Wetland           | 0.5           | 50,000        |
| Mangrove          | 0.5           | 50,000        |
| CoralReef         | 1             | 25,000        |
| Other             | 10            | 100,000       |

## Polygon Registration Requirements

To register a polygon, the following must be provided:

1. **Polygon ID** — 32-byte unique identifier (typically `sha256(project_id + geohash)`)
2. **Geometry IPFS CID** — CID v1 of the full GeoJSON uploaded to IPFS
3. **Bounding Box** — Computed from the full geometry
4. **Area** — Must be within biome-specific min/max bounds
5. **Biome** — Valid biome classification code
6. **Country** — 2-letter ISO 3166-1 alpha-2 code
7. **Project ID** — Reference to the associated project
