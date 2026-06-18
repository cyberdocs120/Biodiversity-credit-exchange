import { NavLink } from "react-router-dom";

const linkStyle: React.CSSProperties = {
  textDecoration: "none",
  color: "#ccc",
  fontWeight: 500,
  fontSize: 15,
  padding: "8px 16px",
  borderRadius: 6,
  transition: "background 0.15s",
};

const activeStyle: React.CSSProperties = {
  ...linkStyle,
  color: "#fff",
  background: "rgba(255,255,255,0.12)",
};

export default function Header() {
  return (
    <header
      style={{
        background: "#1a3a2a",
        padding: "0 24px",
        display: "flex",
        alignItems: "center",
        height: 56,
        gap: 32,
      }}
    >
      <strong style={{ color: "#fff", fontSize: 18, letterSpacing: 1 }}>
        BDCX
      </strong>
      <nav style={{ display: "flex", gap: 4 }}>
        <NavLink
          to="/"
          end
          style={({ isActive }) => (isActive ? activeStyle : linkStyle)}
        >
          Dashboard
        </NavLink>
        <NavLink
          to="/marketplace"
          style={({ isActive }) => (isActive ? activeStyle : linkStyle)}
        >
          Marketplace
        </NavLink>
        <NavLink
          to="/portfolio"
          style={({ isActive }) => (isActive ? activeStyle : linkStyle)}
        >
          Portfolio
        </NavLink>
      </nav>
    </header>
  );
}
