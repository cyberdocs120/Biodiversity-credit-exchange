export interface HabitatPolygon {
  id: string;
  name: string;
  bbox: [number, number, number, number];
  biome: string;
  credits: number;
  areaHa: number;
  country: string;
  status: "active" | "closed";
}

const MOCK_POLYGONS: HabitatPolygon[] = [
  {
    id: "0x0100000000000000000000000000000000000000000000000000000000000001",
    name: "Amazon Basin Reserve",
    bbox: [-73.2, -9.1, -71.8, -7.5],
    biome: "Tropical Rainforest",
    credits: 12500,
    areaHa: 4500,
    country: "BR",
    status: "active",
  },
  {
    id: "0x0100000000000000000000000000000000000000000000000000000000000002",
    name: "Costa Rica Corridor",
    bbox: [-84.8, 9.2, -83.5, 10.1],
    biome: "Tropical Rainforest",
    credits: 8300,
    areaHa: 2800,
    country: "CR",
    status: "active",
  },
  {
    id: "0x0100000000000000000000000000000000000000000000000000000000000003",
    name: "Mongolian Steppe",
    bbox: [103.0, 46.5, 105.5, 48.0],
    biome: "Grassland",
    credits: 4200,
    areaHa: 12000,
    country: "MN",
    status: "active",
  },
  {
    id: "0x0100000000000000000000000000000000000000000000000000000000000004",
    name: "Kenya Mangrove Project",
    bbox: [39.5, -4.8, 40.1, -4.1],
    biome: "Wetland",
    credits: 6100,
    areaHa: 1900,
    country: "KE",
    status: "active",
  },
];

export function usePolygons(): {
  polygons: HabitatPolygon[];
  loading: boolean;
} {
  return { polygons: MOCK_POLYGONS, loading: false };
}
