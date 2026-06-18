import { useEffect, useRef, useState } from "react";
import maplibregl from "maplibre-gl";
import type { HabitatPolygon } from "../hooks/usePolygons";

interface Props {
  polygons: HabitatPolygon[];
  height?: number;
}

export default function PolygonMap({ polygons, height = 400 }: Props) {
  const containerRef = useRef<HTMLDivElement>(null);
  const mapRef = useRef<maplibregl.Map | null>(null);
  const [hovered, setHovered] = useState<HabitatPolygon | null>(null);

  useEffect(() => {
    if (!containerRef.current || mapRef.current) return;

    const map = new maplibregl.Map({
      container: containerRef.current,
      style: "https://basemaps.cartocdn.com/gl/positron-gl-style/style.json",
      center: [0, 20],
      zoom: 1.5,
      attributionControl: false,
    });

    map.addControl(new maplibregl.NavigationControl(), "top-right");
    mapRef.current = map;

    return () => {
      map.remove();
      mapRef.current = null;
    };
  }, []);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !map.isStyleLoaded()) return;

    const sourceId = "polygons";
    if (map.getSource(sourceId)) {
      (map.getSource(sourceId) as maplibregl.GeoJSONSource).setData({
        type: "FeatureCollection",
        features: polygons.map((p) => ({
          type: "Feature",
          properties: {
            id: p.id,
            name: p.name,
            biome: p.biome,
            credits: p.credits,
            areaHa: p.areaHa,
            country: p.country,
            status: p.status,
          },
          geometry: {
            type: "Polygon",
            coordinates: [
              [
                [p.bbox[0], p.bbox[1]],
                [p.bbox[2], p.bbox[1]],
                [p.bbox[2], p.bbox[3]],
                [p.bbox[0], p.bbox[3]],
                [p.bbox[0], p.bbox[1]],
              ],
            ],
          },
        })),
      });
    } else {
      map.addSource(sourceId, {
        type: "geojson",
        data: {
          type: "FeatureCollection",
          features: polygons.map((p) => ({
            type: "Feature",
            properties: {
              id: p.id,
              name: p.name,
              biome: p.biome,
              credits: p.credits,
              areaHa: p.areaHa,
              country: p.country,
              status: p.status,
            },
            geometry: {
              type: "Polygon",
              coordinates: [
                [
                  [p.bbox[0], p.bbox[1]],
                  [p.bbox[2], p.bbox[1]],
                  [p.bbox[2], p.bbox[3]],
                  [p.bbox[0], p.bbox[3]],
                  [p.bbox[0], p.bbox[1]],
                ],
              ],
            },
          })),
        },
      });

      map.addLayer({
        id: "polygons-fill",
        type: "fill",
        source: sourceId,
        paint: {
          "fill-color": [
            "match",
            ["get", "biome"],
            "Tropical Rainforest",
            "#2d7d46",
            "Grassland",
            "#a4c639",
            "Wetland",
            "#4a90d9",
            "#888888",
          ],
          "fill-opacity": 0.35,
        },
      });

      map.addLayer({
        id: "polygons-outline",
        type: "line",
        source: sourceId,
        paint: {
          "line-color": "#1a3a2a",
          "line-width": 1.5,
          "line-opacity": 0.7,
        },
      });
    }

    map.on("mouseenter", "polygons-fill", (e) => {
      map.getCanvas().style.cursor = "pointer";
      if (e.features?.[0]?.properties) {
        const props = e.features[0].properties;
        setHovered({
          id: props.id,
          name: props.name,
          bbox: [0, 0, 0, 0],
          biome: props.biome,
          credits: props.credits,
          areaHa: props.areaHa,
          country: props.country,
          status: props.status,
        });
      }
    });

    map.on("mouseleave", "polygons-fill", () => {
      map.getCanvas().style.cursor = "";
      setHovered(null);
    });
  }, [polygons]);

  return (
    <div style={{ position: "relative", borderRadius: 8, overflow: "hidden" }}>
      <div ref={containerRef} style={{ width: "100%", height }} />
      {hovered && (
        <div
          style={{
            position: "absolute",
            bottom: 12,
            left: 12,
            background: "rgba(255,255,255,0.95)",
            padding: "10px 14px",
            borderRadius: 8,
            fontSize: 13,
            boxShadow: "0 2px 8px rgba(0,0,0,0.15)",
            pointerEvents: "none",
          }}
        >
          <strong>{hovered.name}</strong>
          <br />
          {hovered.biome} · {hovered.areaHa.toLocaleString()} ha
          <br />
          {hovered.credits.toLocaleString()} credits · {hovered.country}
        </div>
      )}
    </div>
  );
}
