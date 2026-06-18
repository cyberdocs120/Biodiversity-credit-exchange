import { Routes, Route } from "react-router-dom";
import Header from "./components/Header";
import DashboardPage from "./pages/DashboardPage";
import MarketplacePage from "./pages/MarketplacePage";
import PortfolioPage from "./pages/PortfolioPage";

export default function App() {
  return (
    <div style={{ minHeight: "100vh", background: "#f5f5f5" }}>
      <Header />
      <main style={{ maxWidth: 1200, margin: "0 auto", padding: "24px 16px" }}>
        <Routes>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/marketplace" element={<MarketplacePage />} />
          <Route path="/portfolio" element={<PortfolioPage />} />
        </Routes>
      </main>
    </div>
  );
}
