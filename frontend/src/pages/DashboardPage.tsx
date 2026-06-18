import { usePolygons } from "../hooks/usePolygons";
import PolygonMap from "../components/PolygonMap";

function StatCard({
  label,
  value,
}: {
  label: string;
  value: string | number;
}) {
  return (
    <div
      style={{
        background: "#fff",
        borderRadius: 10,
        padding: "18px 20px",
        boxShadow: "0 1px 4px rgba(0,0,0,0.06)",
        flex: "1 1 200px",
      }}
    >
      <div style={{ fontSize: 13, color: "#888", marginBottom: 4 }}>
        {label}
      </div>
      <div style={{ fontSize: 26, fontWeight: 700, color: "#1a3a2a" }}>
        {value}
      </div>
    </div>
  );
}

const MOCK_ACTIVITY = [
  { event: "Survey Submitted", polygon: "Amazon Basin Reserve", time: "2h ago" },
  { event: "Proposal Approved", polygon: "Costa Rica Corridor", time: "5h ago" },
  { event: "Credits Minted", polygon: "Amazon Basin Reserve", time: "6h ago" },
  { event: "Order Matched", polygon: "—", time: "12h ago" },
  { event: "Credits Retired", polygon: "Mongolian Steppe", time: "1d ago" },
];

export default function DashboardPage() {
  const { polygons } = usePolygons();

  return (
    <div>
      <h2 style={{ marginTop: 0, marginBottom: 20, color: "#1a3a2a" }}>
        Dashboard
      </h2>

      <div
        style={{
          display: "flex",
          flexWrap: "wrap",
          gap: 16,
          marginBottom: 24,
        }}
      >
        <StatCard label="Total BDCs Minted" value="—" />
        <StatCard label="Active Polygons" value={polygons.length} />
        <StatCard label="Active Orders" value="—" />
        <StatCard label="BDCs Retired" value="—" />
      </div>

      <div style={{ marginBottom: 24 }}>
        <PolygonMap polygons={polygons} height={420} />
      </div>

      <div
        style={{
          background: "#fff",
          borderRadius: 10,
          padding: 20,
          boxShadow: "0 1px 4px rgba(0,0,0,0.06)",
        }}
      >
        <h3 style={{ margin: "0 0 12px", fontSize: 16, color: "#1a3a2a" }}>
          Recent Activity
        </h3>
        <table style={{ width: "100%", borderCollapse: "collapse" }}>
          <thead>
            <tr style={{ textAlign: "left", color: "#888", fontSize: 13 }}>
              <th style={{ padding: "6px 8px" }}>Event</th>
              <th style={{ padding: "6px 8px" }}>Polygon</th>
              <th style={{ padding: "6px 8px" }}>Time</th>
            </tr>
          </thead>
          <tbody>
            {MOCK_ACTIVITY.map((a, i) => (
              <tr key={i} style={{ borderTop: "1px solid #f0f0f0" }}>
                <td style={{ padding: "10px 8px", fontSize: 14 }}>
                  {a.event}
                </td>
                <td style={{ padding: "10px 8px", fontSize: 14 }}>
                  {a.polygon}
                </td>
                <td style={{ padding: "10px 8px", fontSize: 14, color: "#888" }}>
                  {a.time}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
