import { useState } from "react";

const BIOMES = [
  "All Biomes",
  "Tropical Rainforest",
  "Grassland",
  "Wetland",
  "Savanna",
  "Temperate Forest",
];

function OrderCard({
  side,
  orders,
}: {
  side: "Buy" | "Sell";
  orders: { price: number; qty: number; trader: string }[];
}) {
  return (
    <div
      style={{
        flex: 1,
        background: "#fff",
        borderRadius: 10,
        padding: 16,
        boxShadow: "0 1px 4px rgba(0,0,0,0.06)",
      }}
    >
      <h3
        style={{
          margin: "0 0 12px",
          fontSize: 15,
          color: side === "Buy" ? "#2d7d46" : "#c0392b",
        }}
      >
        {side} Orders
      </h3>
      {orders.length === 0 ? (
        <p style={{ color: "#aaa", fontSize: 14 }}>No orders yet.</p>
      ) : (
        <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
          {orders.map((o, i) => (
            <div
              key={i}
              style={{
                display: "flex",
                justifyContent: "space-between",
                fontSize: 14,
                padding: "8px 0",
                borderTop: "1px solid #f0f0f0",
              }}
            >
              <span>
                {o.qty} credits @ ${o.price.toFixed(2)}
              </span>
              <span style={{ color: "#888" }}>
                {o.trader.slice(0, 6)}…
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default function MarketplacePage() {
  const [biomeFilter, setBiomeFilter] = useState("All Biomes");

  return (
    <div>
      <h2 style={{ marginTop: 0, marginBottom: 20, color: "#1a3a2a" }}>
        Marketplace
      </h2>

      <div
        style={{
          display: "flex",
          gap: 20,
          marginBottom: 20,
          flexWrap: "wrap",
        }}
      >
        <div
          style={{
            background: "#fff",
            borderRadius: 10,
            padding: "12px 20px",
            boxShadow: "0 1px 4px rgba(0,0,0,0.06)",
            flex: "1 1 200px",
          }}
        >
          <div style={{ fontSize: 12, color: "#888" }}>Best Bid</div>
          <div style={{ fontSize: 22, fontWeight: 700, color: "#2d7d46" }}>
            —
          </div>
        </div>
        <div
          style={{
            background: "#fff",
            borderRadius: 10,
            padding: "12px 20px",
            boxShadow: "0 1px 4px rgba(0,0,0,0.06)",
            flex: "1 1 200px",
          }}
        >
          <div style={{ fontSize: 12, color: "#888" }}>Best Ask</div>
          <div style={{ fontSize: 22, fontWeight: 700, color: "#c0392b" }}>
            —
          </div>
        </div>
        <div style={{ alignSelf: "flex-end", marginLeft: "auto" }}>
          <label style={{ fontSize: 13, color: "#888", marginRight: 8 }}>
            Biome:
          </label>
          <select
            value={biomeFilter}
            onChange={(e) => setBiomeFilter(e.target.value)}
            style={{
              padding: "6px 12px",
              borderRadius: 6,
              border: "1px solid #ccc",
              fontSize: 14,
            }}
          >
            {BIOMES.map((b) => (
              <option key={b} value={b}>
                {b}
              </option>
            ))}
          </select>
        </div>
      </div>

      <div style={{ display: "flex", gap: 16, flexWrap: "wrap" }}>
        <OrderCard side="Buy" orders={[]} />
        <OrderCard side="Sell" orders={[]} />
      </div>
    </div>
  );
}
