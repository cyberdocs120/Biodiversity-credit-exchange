# Biodiversity Credit Methodology

## Overview

BDCX credits represent verified biodiversity impact measured through the **Biodiversity Score Index (BSI)**, a multi-metric composite index that quantifies ecosystem health across five dimensions.

## BSI Formula

```
BSI = 0.30 × SR_norm + 0.25 × SDI_norm + 0.20 × CC_norm + 0.15 × IUCN_norm + 0.10 × FQ_norm
```

### Variable Definitions

| Variable       | Weight | Description                                          | Range      |
|----------------|--------|------------------------------------------------------|------------|
| `SR_norm`      | 0.30   | Species Richness (normalized 0–1)                   | 0.0 – 1.0  |
| `SDI_norm`     | 0.25   | Shannon Diversity Index (normalized 0–1)            | 0.0 – 1.0  |
| `CC_norm`      | 0.20   | Canopy Cover / vegetation density (normalized 0–1)  | 0.0 – 1.0  |
| `IUCN_norm`    | 0.15   | IUCN Red List species presence (normalized 0–1)     | 0.0 – 1.0  |
| `FQ_norm`      | 0.10   | Floristic Quality Index (normalized 0–1)            | 0.0 – 1.0  |

All sub-indices are normalized to a 0–1 scale before weighting.

## Credit Calculation

```
BDC_credits = (BSI_current - BSI_baseline) × polygon_area_ha × quality_multiplier
```

Where:
- `BSI_baseline` — BSI measured at project registration (pre-intervention)
- `BSI_current` — BSI measured at survey time (post-intervention)
- `polygon_area_ha` — Habitat polygon area in hectares
- `quality_multiplier` — Adjustment factor based on project quality attributes

### Numerical Example

**Scenario:** Tropical forest restoration project

```
BSI_baseline  = 0.28  (degraded cattle pasture)
BSI_current   = 0.64  (after 5 years of restoration)
Area          = 4,500 ha
Multiplier    = 1.0   (standard project)

BDC_credits   = (0.64 - 0.28) × 4,500 × 1.0
              = 0.36 × 4,500
              = 1,620 credits
```

### Additional Examples

| Baseline | Current | Area (ha) | Multiplier | Credits | Scenario                         |
|----------|---------|-----------|------------|---------|----------------------------------|
| 0.15     | 0.72    | 10,000    | 1.2        | 6,840   | IUCN priority reforestation      |
| 0.45     | 0.58    | 2,000     | 1.5        | 390     | FPIC community-led conservation  |
| 0.60     | 0.65    | 8,000     | 0.8        | 320     | Restored wetland (diminishing)   |
| 0.05     | 0.81    | 500       | 1.2        | 456     | Small high-impact mangrove       |

## Quality Multiplier Table

| Multiplier | Condition                          | Description                                |
|------------|------------------------------------|--------------------------------------------|
| 1.0        | Standard                           | Default for qualifying projects            |
| 1.2        | IUCN Priority                      | Project in IUCN Red List priority area     |
| 1.5        | FPIC                               | Full Free Prior and Informed Consent from indigenous communities |
| 0.8        | Restored                           | Previously restored land (diminishing additionality) |
| 1.3        | Verified Carbon + Biodiversity     | Joint crediting with verified carbon standard |
| 1.1        | Endangered Ecosystem               | CR/EN ecosystem classification             |

## Methodology Versioning

All methodologies follow the naming scheme: `BDCX-{TYPE}-v{major}.{minor}`

| Version          | Type         | Description                            |
|------------------|--------------|----------------------------------------|
| BDCX-TF-v1.0     | TropicalForest | TerraFirma methodology                |
| BDCX-MG-v1.0     | Mangrove       | Mangrove restoration methodology      |
| BDCX-GS-v1.0     | Grassland      | Grassland/savanna methodology         |
| BDCX-WL-v1.0     | Wetland        | Wetland peatland methodology          |
| BDCX-TP-v1.0     | TemperateForest | Temperate forest methodology          |
| BDCX-CR-v1.0     | CoralReef      | Coral reef restoration methodology    |
| BDCX-OT-v1.0     | Other          | Generic methodology for other biomes  |

## Oracle Cross-Validation Tolerances

When multiple oracle types submit surveys for the same polygon, cross-validation tolerances apply:

| Oracle Pair                    | BSI Tolerance | Description                          |
|--------------------------------|---------------|--------------------------------------|
| eDNA Lab ↔ Field Survey        | ±0.05         | Genetic vs physical species count    |
| Satellite Imagery ↔ Camera Trap AI | ±0.08     | Remote sensing vs ground-truth       |
| Camera Trap AI ↔ Field Survey  | ±0.06         | AI identification vs expert survey   |
| Acoustic Sensor ↔ Field Survey | ±0.07         | Audio analysis vs manual assessment  |
| Satellite Imagery ↔ Field Survey | ±0.10       | Coarse vs fine resolution            |

If a survey's BSI deviates beyond tolerance from another oracle's recent survey of the same polygon, a dispute is automatically flagged.

## Survey Requirements

| Requirement              | Value                        |
|--------------------------|------------------------------|
| Minimum oracle threshold | 2-of-3 signatures (configurable) |
| Maximum survey age       | 90 days before stale         |
| Re-survey interval       | Minimum 30 days              |
| Polygon minimum area     | 1 ha                         |
| Polygon maximum area     | 100,000 ha                   |
