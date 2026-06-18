import { useState } from "react";
import { usePolygons } from "../hooks/usePolygons";
import PolygonMap from "../components/PolygonMap";

const MOCK_RETIREMENTS = [
  {
    id: "0xabcd…ef01",
    polygon: "Amazon Basin Reserve",
    credits: 500,
    date: "2026-05-10",
  },
  {
    id: "0x1234…5678",
    polygon: "Mongolian Steppe",
    credits: 200,
    date: "2026-05-08",
  },
];

export default function PortfolioPage() {
  const { polygons } = usePolygons();
  const [claimId, setClaimId] = useState("");
  const [verified, setVerified] = useState<boolean | null>(null);

  const handleVerify = () => {
    setVerified(claimId.trim().length > 0);
  };

  return (
    <div>
      <h2 style={{ marginTop: 0, marginBottom: 20, color: "#1a3a2a" }}>
        Portfolio
      </h2>

      <div
        style={{
          display: "flex",
          gap: 16,
          marginBottom: 24,
          flexWrap: "wrap",
        }}
      >
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
            BDC Holdings
          </div>
          <div style={{ fontSize: 26, fontWeight: 700, color: "#1a3a2a" }}>
            —
          </div>
        </div>
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
            Total Retired
          </div>
          <div style={{ fontSize: 26, fontWeight: 700, color: "#1a3a2a" }}>
            700
          </div>
        </div>
      </div>

      <div style={{ marginBottom: 24 }}>
        <PolygonMap polygons={polygons} height={300} />
      </div>

      <div
        style={{
          background: "#fff",
          borderRadius: 10,
          padding: 20,
          boxShadow: "0 1px 4px rgba(0,0,0,0.06)",
          marginBottom: 24,
        }}
      >
        <h3 style={{ margin: "0 0 12px", fontSize: 16, color: "#1a3a2a" }}>
          Retirement History
        </h3>
        {MOCK_RETIREMENTS.length === 0 ? (
          <p style={{ color: "#aaa", fontSize: 14 }}>No retirements yet.</p>
        ) : (
          <table style={{ width: "100%", borderCollapse: "collapse" }}>
            <thead>
              <tr style={{ textAlign: "left", color: "#888", fontSize: 13 }}>
                <th style={{ padding: "6px 8px" }}>Receipt ID</th>
                <th style={{ padding: "6px 8px" }}>Polygon</th>
                <th style={{ padding: "6px 8px" }}>Credits</th>
                <th style={{ padding: "6px 8px" }}>Date</th>
              </tr>
            </thead>
            <tbody>
              {MOCK_RETIREMENTS.map((r, i) => (
                <tr key={i} style={{ borderTop: "1px solid #f0f0f0" }}>
                  <td style={{ padding: "10px 8px", fontSize: 14, fontFamily: "monospace" }}>{r.id}</td>
                  <td style={{ padding: "10px 8px", fontSize: 14 }}>{r.polygon}</td>
                  <td style={{ padding: "10px 8px", fontSize: 14 }}>{r.credits}</td>
                  <td style={{ padding: "10px 8px", fontSize: 14, color: "#888" }}>{r.date}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
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
          Claim Verification
        </h3>
        <div style={{ display: "flex", gap: 8 }}>
          <input
            type="text"
            placeholder="Enter claim / receipt ID…"
            value={claimId}
            onChange={(e) => {
              setClaimId(e.target.value);
              setVerified(null);
            }}
            style={{
              flex: 1,
              padding: "8px 12px",
              borderRadius: 6,
              border: "1px solid #ccc",
              fontSize: 14,
            }}
          />
          <button
            onClick={handleVerify}
            style={{
              padding: "8px 20px",
              borderRadius: 6,
              border: "none",
              background: "#1a3a2a",
              color: "#fff",
              fontSize: 14,
              fontWeight: 600,
              cursor: "pointer",
            }}
          >
            Verify
          </button>
        </div>
        {verified !== null && (
          <div
            style={{
              marginTop: 12,
              padding: "10px 14px",
              borderRadius: 6,
              background: verified ? "#e6f4ea" : "#fce8e6",
              color: verified ? "#1e7e34" : "#c5221f",
              fontSize: 14,
            }}
          >
            {verified
              ? "✓ Claim verified on-chain."
              : "✗ Receipt not found. Please check the ID."}
          </div>
        )}
      </div>
    </div>
  );
}
