# 🌿 Biodiversity Credit Exchange (BDCX)

> **Tokenize conservation outcomes. Trade biodiversity. Retire on habitat polygons.**

<div align="center">

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.81+-orange.svg)](https://www.rust-lang.org)
[![Soroban](https://img.shields.io/badge/Soroban-22.x-blueviolet.svg)](https://soroban.stellar.org)
[![Stellar](https://img.shields.io/badge/Stellar-Network-7B1FA2.svg)](https://stellar.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](#contribute)

</div>

**1 Verified Biodiversity Unit = 1 Token · Mint → Approve → Trade → Retire · Built on Stellar Soroban**

Tokenize, trade, and retire biodiversity conservation outcomes as first-class Stellar (Soroban) smart contract assets. Each BDC token is cryptographically bound to a specific habitat polygon, verified through multi-stakeholder ecological MRV (Monitoring, Reporting, and Verification), and retired against on-chain geospatial claims — enabling verifiable, transparent, and liquid markets for ecological restoration at near-zero transaction cost.

---

## 📦 Table of Contents

- [Why This Matters](#-why-this-matters)
- [Project Structure](#-project-structure)
- [Market Opportunity](#-market-opportunity)
- [Vision & Pain Points](#-vision--pain-points)
- [How It Works](#-how-it-works)
- [System Architecture](#-system-architecture)
- [User Personas](#-user-personas)
- [Use Cases](#-use-cases)
- [Smart Contracts](#-smart-contracts)
  - [BDC Token Contract](#1-bdc-token-contract)
  - [MRV Oracle Contract](#2-mrv-oracle-contract)
  - [Approval Governance Contract](#3-approval-governance-contract)
  - [Retirement Registry Contract](#4-retirement-registry-contract)
- [Multi-Stakeholder Approval Workflow](#-multi-stakeholder-approval-workflow)
- [MRV Data Model & IPFS Integration](#-mrv-data-model--ipfs-integration)
- [Habitat Polygon System](#-habitat-polygon-system)
- [Credit Methodology](#-credit-methodology)
- [Tokenomics & Fee Model](#-tokenomics--fee-model)
- [Data Model](#-data-model)
- [Event & Error Reference](#-event--error-reference)
- [Cross-Contract Interaction Flow](#-cross-contract-interaction-flow)
- [Governance Model](#-governance-model)
- [Regulatory & Standards Alignment](#-regulatory--standards-alignment)
- [Technical Stack](#-technical-stack)
- [Comparison With Existing Solutions](#-comparison-with-existing-solutions)
- [Getting Started](#-getting-started)
- [API Reference](#-api-reference)
- [Security & Risk](#-security--risk)
- [Roadmap](#-roadmap)
- [FAQ](#-faq)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🌍 Why This Matters

Biodiversity loss is accelerating at an unprecedented rate. The world is facing its **sixth mass extinction**, with **1 million species** at risk of extinction (IPBES 2019). Current conservation funding — approximately **$50B/year** — falls short by **$700B/year** of what is needed to reverse biodiversity loss (UNEP).

Biodiversity credits offer a market-based mechanism to close this gap, but existing systems are fragmented, opaque, and lack rigorous verification:

| Problem | Impact |
|---------|--------|
| No standardized biodiversity unit | Markets cannot form; credits are bespoke and non-comparable |
| Manual MRV (paper-based surveys) | High cost, low frequency, prone to fraud |
| Single-point verification | Conflicts of interest; no community or ecologist oversight |
| No on-chain retirement | Double-counting of conservation claims; no audit trail |
| Illiquid secondary markets | Capital cannot flow efficiently to high-impact projects |

**BDCX solves all of these** by bringing biodiversity credits onto the Stellar network with multi-stakeholder MRV, on-chain polygon-anchored retirements, and permissionless trading.

---

## 📁 Project Structure

```
biodiversity-credit-exchange/
├── Cargo.toml                          # Workspace root (multi-contract build)
├── README.md                           # This document
├── LICENSE                             # MIT license
├── .gitignore
│
├── contracts/                          # Soroban smart contracts
│   ├── bdc-token/                      # Biodiversity credit token (mint/burn/transfer)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # Core token logic
│   │       ├── storage.rs              # Storage keys & helpers
│   │       ├── types.rs                # Data structures
│   │       ├── errors.rs               # Error definitions
│   │       ├── metadata.rs             # IPFS metadata resolution
│   │       └── test.rs                # Unit tests
│   │
│   ├── mrv-oracle/                     # Ecological MRV data ingestion
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # Oracle core logic
│   │       ├── storage.rs              # Oracle state
│   │       ├── types.rs                # Survey data types
│   │       ├── errors.rs
│   │       ├── verifiers.rs            # Plausibility & cross-validation
│   │       ├── ipfs.rs                 # IPFS content addressing helpers
│   │       └── test.rs
│   │
│   ├── approval-gov/                   # Multi-stakeholder approval governance
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # Approval workflow engine
│   │       ├── storage.rs
│   │       ├── types.rs                # Stakeholder roles, proposals
│   │       ├── errors.rs
│   │       ├── voting.rs               # Weighted voting logic
│   │       └── test.rs
│   │
│   ├── retirement/                     # Polygon-anchored retirements
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                  # Retirement engine
│   │       ├── storage.rs
│   │       ├── types.rs                # Polygon geometry, receipts
│   │       ├── errors.rs
│   │       ├── geometry.rs             # Simple polygon ops (point-in-polygon)
│   │       ├── merkle.rs               # Merkle proof for batch claims
│   │       └── test.rs
│   │
│   └── marketplace/                    # Spot trading + CfD engine
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── storage.rs
│           ├── types.rs
│           ├── errors.rs
│           ├── order_book.rs           # Price-time priority queue
│           ├── matching_engine.rs      # Order matching
│           └── test.rs
│
├── tests/                              # Integration tests
│   └── integration/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs                  # End-to-end sandbox scenarios
│
├── scripts/                            # DevOps & deployment
│   ├── deploy.sh                       # Testnet/mainnet deployment
│   ├── setup.sh                        # Local sandbox bootstrap
│   └── test.sh                         # Full test runner
│
├── docs/                               # Extended documentation
│   ├── architecture.md                 # Deep-dive architecture
│   ├── methodology.md                  # Biodiversity credit methodology
│   ├── polygon-spec.md                 # On-chain geometry spec
│   └── oracle-spec.md                  # MRV oracle network spec
│
└── frontend/                           # Web dashboard (React)
    ├── package.json
    ├── tsconfig.json
    ├── vite.config.ts
    ├── index.html
    └── src/
        ├── App.tsx
        ├── components/                 # Reusable UI components
        ├── hooks/                      # Soroban SDK bindings
        └── pages/                      # Dashboard views
```

### Workspace Architecture

The project is a **Cargo workspace** with one crate per contract, plus an integration test crate:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         Cargo Workspace                                 │
│   root Cargo.toml with [workspace] members                              │
│                                                                         │
│   ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌───────────┐  │
│   │ bdc-token    │  │ mrv-oracle   │  │ approval-gov │  │marketplace│  │
│   │ (lib)        │  │ (lib)        │  │ (lib)        │  │ (lib)     │  │
│   └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └─────┬─────┘  │
│          │                 │                  │                │        │
│          └────────────┬────┴──────────────────┴────────────────┘        │
│                       │                                                 │
│             ┌─────────▼──────────┐                                      │
│             │   retirement       │                                      │
│             │   (lib)            │                                      │
│             └────────────────────┘                                      │
│                                                                         │
│   ┌──────────────────────────────────────────────────────────────┐      │
│   │  tests/integration                                           │      │
│   │  (integration tests across all 5 contracts)                   │      │
│   └──────────────────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────────────────┘
```

Each contract crate compiles to a separate `.wasm` blob, deployable independently. The integration test crate exercises cross-contract calls in a sandboxed Soroban environment.

---

## 📊 Market Opportunity

| Metric | Value | Source |
|--------|-------|--------|
| Global biodiversity finance gap | ~$700B/year | UNEP 2023 |
| Current biodiversity credit market | ~$15–25M (early) | Ecosystem Marketplace |
| Projected market size (2035) | $50–200B | McKinsey, NatureFinance |
| Voluntary carbon market (comparison) | ~$2B (2023), projected $100B+ | TSVCM |
| Corporate nature commitments (SBTN) | 200+ companies | SBTN 2024 |
| Average biodiversity credit price | $15–50/credit (varies by methodology) | Various registries |
| Our target latency | ~5 seconds | Stellar |
| Traditional registry settlement | 60–180 days | Current best practice |

The biodiversity credit market sits where carbon markets were in 2010 — fragmented, manual, and opaque. BDCX provides the on-chain infrastructure for a **$100B+ market** projected by 2035.

---

## 🎯 Vision & Pain Points

| Pain Point | Magnitude | How BDCX Fixes It |
|-----------|-----------|-------------------|
| **No standardized unit** | 50+ methodologies, no fungibility | BDC token with standardized metadata: `1 credit = 1 verified biodiversity unit` |
| **Manual ecological surveys** | Costly, annual, easy to fake | Multi-oracle MRV pipeline: eDNA + camera traps + satellite imagery → IPFS → on-chain |
| **Single-actor verification** | Conflict of interest, greenwashing | Multi-stakeholder approval: ecologists + local communities + independent auditors |
| **No polygon-level traceability** | Credits untethered from geography | Every credit references a GPS habitat polygon on-chain; retirement consumes specific geometry |
| **Double counting** | Pervasive in voluntary markets | Immutable on-chain retirement; one BDC burned = one claim verified |
| **Slow settlement** | 60–180 day registry cycles | Atomic smart contract settlement in ~5 seconds |
| **High intermediation costs** | 10–25% fees to brokers/middlemen | Direct P2P settlement with <1% protocol fee |
| **No secondary liquidity** | >80% of credits never traded after issuance | Automated order-book marketplace |
| **Community exclusion** | Indigenous/local communities sidelined | Weighted voting in approval governance; community seats with veto power |
| **Fragmented registries** | Verra, Gold Standard, Plan Vivo, etc. | Unified on-chain standard; bridge adapters for legacy registries |

We move biodiversity credits from **PDF reports + spreadsheets + email negotiations** onto the **Stellar network** — fast (<$0.00001/tx), cheap, carbon-friendly (Stellar is 99.99% more efficient than PoW chains), and globally accessible.

---

## 🔄 How It Works

### Lifecycle (End-to-End)

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           PHYSICAL WORLD                                     │
│   ┌──────────────────────────────────────────────────────────────────────┐   │
│   │  Ecological Site (Habitat Polygon)                                    │   │
│   │  ┌─────────────────────────────────────────────┐                     │   │
│   │  │  GPS Polygon: [[lat1,lon1],[lat2,lon2],...] │                     │   │
│   │  │  Area: 1,250 ha  ·  Biome: Tropical Forest  │                     │   │
│   │  │  Baseline: 0.42 BSI (Biodiversity Score)    │                     │   │
│   │  └─────────────────────────────────────────────┘                     │   │
│   └──────────────────────────────────────────────────────────────────────┘   │
│                              │                                                │
│          eDNA samples · Camera traps · Acoustic sensors · Field surveys      │
│                              ▼                                                │
└──────────────────────────────────────────────────────────────────────────────┘
                              │ IPFS (encrypted + content-addressed)
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           MRV ORACLE LAYER                                   │
│   ┌──────────────────────────────────────────────────────────────────────┐   │
│   │  Oracle Network (N-of-M threshold)                                   │   │
│   │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐   │   │
│   │  │  eDNA    │ │ Camera   │ │ Satellite│ │ Field    │ │ Acoustic │   │   │
│   │  │  Lab     │ │ Trap AI  │ │ Imagery  │ │ Surveyor │ │ Sensor   │   │   │
│   │  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘   │   │
│   │                        │  IPFS hash verification                      │   │
│   │                        │  Cross-validation (≥3 oracles must agree)    │   │
│   └──────────────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────┬───────────────────────────────────────────┘
                                   │ submit_survey(polygon_id, ipfs_hash, sigs)
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        MULTI-STAKEHOLDER APPROVAL                             │
│   ┌──────────────────────────────────────────────────────────────────────┐   │
│   │  Approval Governance                                                  │   │
│   │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐               │   │
│   │  │  Ecologists  │  │  Local       │  │  Independent │               │   │
│   │  │  (weight=3)  │  │  Community   │  │  Auditor     │               │   │
│   │  │              │  │  (weight=2)  │  │  (weight=1)  │               │   │
│   │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘               │   │
│   │         └────────┬────────┴─────────┬───────┘                        │   │
│   │                  │ Approve/Reject    │                                │   │
│   │                  ▼                    ▼                                │   │
│   │        ┌─────────────────────────────────────┐                        │   │
│   │        │  Threshold: weighted sum ≥ 4 of 6   │                        │   │
│   │        │  Community veto: always honored      │                        │   │
│   │        └─────────────────────────────────────┘                        │   │
│   └──────────────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────┬───────────────────────────────────────────┘
                                   │ on_approved(polygon_id, survey_hash)
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        SOROBAN SMART CONTRACTS                                │
│                                                                               │
│   ┌──────────────┐     ┌──────────────┐     ┌──────────────────┐             │
│   │  BDC Token   │◄────│  MRV Oracle  │────►│  Marketplace     │             │
│   │  (SEP-41)    │     │  Handler     │     │  (Spot + CfD)    │             │
│   │              │     │              │     │                  │             │
│   │  mint()      │     │  submit_     │     │  place_order()   │             │
│   │  burn()      │     │  survey()    │     │  match_orders()  │             │
│   │  transfer()  │     │  verify_     │     │  auto_match()    │             │
│   │  token_uri() │     │  ipfs()      │     │  settle()        │             │
│   └──────┬───────┘     │  dispute()   │     └────────┬─────────┘             │
│          │             └──────┬───────┘              │                       │
│          │                    │                       │                       │
│          │    ┌───────────────▼──────────────┐       │                       │
│          │    │  Approval Governance          │       │                       │
│          │    │  ──────────────────────────── │       │                       │
│          │    │  propose() · vote() · veto()  │       │                       │
│          │    │  on_approved() · on_rejected()│       │                       │
│          │    └───────────────────────────────┘       │                       │
│          │                                             │                       │
│          │              ┌───────────────────┐         │                       │
│          ├──────────────│  Retirement       │◄────────┘                       │
│          │              │  Registry         │                                 │
│          │              │  ──────────────── │                                 │
│          │              │  retire()         │                                 │
│          │              │  get_receipt()    │                                 │
│          │              │  verify_claim()   │                                 │
│          │              │  prove_polygon()  │                                 │
│          └──────────────└───────────────────┘                                 │
│                                                                               │
│   ┌──────────────────────────────────────────────────────────────────────┐   │
│   │                  Stellar Blockchain Layer                              │   │
│   │  Finality: ~5s  ·  Fee: ~$0.00001  ·  Carbon: ~0.001 gCO₂/tx        │   │
│   └──────────────────────────────────────────────────────────────────────┘   │
└──────────────────────────────┬───────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                         END USERS                                            │
│   ┌───────────┐   ┌───────────┐   ┌───────────┐   ┌───────────┐            │
│   │  Project  │   │  Corporate│   │  Trader   │   │  Auditor  │            │
│   │  Developer│   │  Buyer    │   │  Market   │   │  Verifier │            │
│   └───────────┘   └───────────┘   └───────────┘   └───────────┘            │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Step-by-Step Lifecycle

| # | Step | Actor | Action | On-Chain Effect |
|---|------|-------|--------|-----------------|
| 1 | **Survey** | Field ecologist | Collects eDNA, deploys camera traps, records transect data | ─ (off-chain) |
| 2 | **Upload** | Survey team | Hashes raw data → IPFS; generates metadata manifest | IPFS CID created |
| 3 | **Oracle ingest** | MRV oracle nodes | Each oracle fetches IPFS data, runs independent analysis, signs hash | N attestations stored |
| 4 | **Verify** | MRV Handler | N-of-M threshold check; cross-validation passes | `ValidatedSurvey` emitted |
| 5 | **Propose** | Project developer | Submits polygon_id + survey_hash to approval governance | `ProposalCreated` emitted |
| 6 | **Vote** | Ecologists / Community / Auditors | Each stakeholder group votes (weighted); community can veto | Votes recorded |
| 7 | **Approve** | Approval Governance | Threshold met → calls bdc-token.mint() | BDC tokens minted to polygon |
| 8 | **Trade** | Market participants | Place buy/sell orders on marketplace | Order book updated |
| 9 | **Match** | Market | Orders matched at crossing prices | BDC transferred; yUSDC settled |
| 10 | **Retire** | Corporate buyer | Calls retire(polygon_id, token_ids[], claim_data) | BDCs burned; receipt with polygon emitted |
| 11 | **Verify** | Auditor | verify_claim(claim_id) → returns polygon, receipt, chain of custody | Immutable proof |

---

## 🏗 System Architecture

### Layer Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                            APPLICATION LAYER                                     │
│   ┌──────────────┐   ┌──────────────┐   ┌──────────────┐   ┌──────────────────┐ │
│   │ Web Dashboard │   │   CLI Tool   │   │  REST API    │   │  Geo-Explorer    │ │
│   │ (React)       │   │ (soroban)    │   │  (indexer)   │   │  (polygon map)   │ │
│   └──────┬───────┘   └──────┬───────┘   └──────┬───────┘   └────────┬─────────┘ │
└──────────┼──────────────────┼──────────────────┼────────────────────┼────────────┘
           │                  │                  │                    │
┌──────────▼──────────────────▼──────────────────▼────────────────────▼────────────┐
│                            CONTRACT LAYER (Soroban)                                │
│                                                                                    │
│   ┌──────────────────┐   ┌──────────────────┐   ┌──────────────────────────┐      │
│   │  BDC Token       │   │  MRV Oracle      │   │  Marketplace + CfD      │      │
│   │  ─────────────── │   │  ─────────────── │   │  ────────────────────── │      │
│   │  balance_of()    │   │  submit_survey() │   │  place_order()          │      │
│   │  mint()          │   │  register_       │   │  match_orders()         │      │
│   │  burn()          │   │  oracle_node()   │   │  auto_match()           │      │
│   │  transfer()      │   │  verify_ipfs()   │   │  settle()               │      │
│   │  token_uri()     │   │  dispute()       │   │                         │      │
│   │  total_supply()  │   │  set_threshold() │   │  open_cfd()             │      │
│   │  credits_by_     │   │  set_verifier()  │   │  close_cfd()            │      │
│   │  polygon()       │   └──────────────────┘   │  liquidate()            │      │
│   └──────────────────┘                          └──────────────────────────┘      │
│                                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────────┐    │
│   │  Approval Governance                                                     │    │
│   │  ─────────────────────                                                   │    │
│   │  propose() · vote() · veto() · on_approved() · on_rejected()            │    │
│   │  set_weights() · set_threshold() · register_stakeholder()                │    │
│   └─────────────────────────────────────────────────────────────────────────┘    │
│                                                                                    │
│   ┌─────────────────────────────────────────────────────────────────────────┐    │
│   │  Retirement Registry                                                      │    │
│   │  ─────────────────────                                                    │    │
│   │  retire() · get_receipt() · verify_retirement() · prove_claim()          │    │
│   │  prove_polygon_containment() · get_retired_polygons()                    │    │
│   └─────────────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────┬─────────────────────────────────────────────┘
                                       │ cross-contract calls (Soroban env)
┌─────────────────────────────────────▼─────────────────────────────────────────────┐
│                            STELLAR CORE                                           │
│   • Consensus: Stellar Consensus Protocol (SCP)                                   │
│   • Finality: 3–5 seconds                                                         │
│   • Fee: ~0.00001 XLM (~$0.000002)                                                │
│   • Throughput: thousands of ops/sec                                              │
└───────────────────────────────────────────────────────────────────────────────────┘
```

### Data Flow

```
                    ┌─────────────────────────────────────┐
                    │         MRV Oracle Network            │
                    │  ┌──────┐  ┌──────┐  ┌──────┐       │
                    │  │eDNA  │  │Cam AI│  │Sat.  │  ...   │
                    │  │ Lab  │  │      │  │Imag. │       │
                    │  └──┬───┘  └──┬───┘  └──┬───┘       │
                    │     │         │         │            │
                    │     └────┬────┴─────────┘            │
                    │          │ threshold sign             │
                    └──────────┼───────────────────────────┘
                               │
                  ┌────────────▼────────────┐
                  │  MRV Oracle Handler      │
                  │  validates N-of-M sigs   │
                  │  cross-validates data    │
                  │  emits ValidatedSurvey   │
                  └────────────┬────────────┘
                               │ cross_contract_call()
                               ▼
                  ┌────────────────────────────┐
                  │  Approval Governance        │
                  │  Creates Proposal           │
                  │  Awaits multi-stakeholder   │
                  │  weighted vote             │
                  └────────────┬───────────────┘
                               │ on_approved()
                               ▼
                  ┌────────────────────────────┐
                  │  BDC Token Contract         │
                  │  mints N credits per        │
                  │  verified biodiversity unit │
                  │  assigns polygon_id         │
                  └────────────┬───────────────┘
                               │
                    ┌──────────┴──────────┐
                    ▼                     ▼
          ┌─────────────────┐   ┌──────────────────────┐
          │ Marketplace     │   │ Retirement Registry   │
          │ Spot + CfD      │   │ Burn + Polygon Claim  │
          └─────────────────┘   └──────────────────────┘
```

---

## 👥 User Personas

### 1. Conservation Project Developer
- **Needs**: Monetize verified ecological restoration outcomes; attract capital for long-term conservation
- **Uses**: Submits survey data via MRV oracle pipeline; receives BDC tokens upon multi-stakeholder approval; sells on market or holds
- **Win**: Unlocks revenue for conservation, not just carbon; transparent verification builds trust with buyers

### 2. Corporate Biodiversity Buyer (SBTN / TNFD)
- **Needs**: Meet Science-Based Targets for Nature (SBTN); TNFD disclosure requirements; verifiable biodiversity claims
- **Uses**: Buys BDC tokens on marketplace; retires against specific habitat polygons matching their supply chain geography; publishes retirement certificates with polygon proofs
- **Win**: Defensible biodiversity claims with on-chain polygon geometry; no double-counting; TNFD-aligned reporting

### 3. Ecological MRV Oracle Operator
- **Needs**: Incentivized to provide accurate survey analysis; reputation for data quality
- **Uses**: Runs eDNA analysis pipeline, camera trap AI, or satellite imagery processing; stakes tokens to signal quality; earns fees per validated survey
- **Win**: Staking rewards; data marketplace; scientific impact

### 4. Independent Ecologist / Auditor
- **Needs**: Scientific integrity; ability to reject fraudulent claims
- **Uses**: Reviews survey data on IPFS; votes on approval proposals with weighted authority; can trigger disputes
- **Win**: Scientific rigor enforced on-chain; weighted voting power proportional to expertise

### 5. Local / Indigenous Community Representative
- **Needs**: Free, Prior, and Informed Consent (FPIC); veto power over projects on traditional lands
- **Uses**: Votes on proposals affecting their territory; exercises community veto; receives benefit-sharing payments
- **Win**: Self-determination encoded in smart contracts; veto power cannot be overridden

### 6. Regulator / Accreditation Body
- **Needs**: Immutable audit trail; methodology compliance; jurisdictional oversight
- **Uses**: Explorer queries; `verify_retirement()` calls; chain-of-custody audits for entire credit lifecycle
- **Win**: Transparent by construction; cryptographic proofs for each credit; methodology parameters verifiable on-chain

---

## 🎬 Use Cases

### Use Case 1: Tropical Forest Restoration — Amazon Basin

```
4,500 ha degraded pasture being restored to Amazonian terra firme forest

Project:
  → Baseline biodiversity score: 0.28 BSI (Biodiversity Score Index)
  → 3-year restoration: 1,200,000 native saplings planted
  → Year 3 survey: BSI = 0.64 (+128% improvement)

MRV Pipeline:
  → 36 eDNA samples (soil + water) → 2,400 species detected
  → 120 camera trap nights → 47 mammal species (3 IUCN threatened)
  → 12 acousic sensor deployments → bird diversity index +42%
  → 3 satellite imagery epochs → canopy cover 12% → 38%

Tokenization:
  → △BSI = 0.36 biodiversity units gained
  → Area multiplier: 4,500 ha × 0.36 = 1,620 BDC tokens minted
  → At $40/BDC = $64,800 annual conservation revenue
  → 20-year projected revenue: $1.3M (covers management + generates surplus)
```

### Use Case 2: Corporate TNFD Disclosure — Agricultural Supply Chain

```
Global food company with soy sourcing from Brazilian Cerrado
  → TNFD requires "avoid, reduce, restore, transform" framework
  → Supply chain overlaps with 12 priority habitat polygons

  → Buys 50,000 BDC tokens from Cerrado restoration projects
  → Retires credits against specific polygons covering 125,000 ha
  → Publishes retirement certificates with polygon proofs in annual TNFD report
  → Auditor verifies: 1) credits exist, 2) polygons match, 3) not double-counted

  Result: Defensible TNFD disclosure with on-chain evidence
```

### Use Case 3: Community-Managed Mangrove Conservation — Indo-Pacific

```
Local community manages 2,800 ha of mangrove forest in Sulawesi

  → Baseline: 0.55 BSI, high carbon stock, critical fisheries nursery
  → Community votes to enroll in BDCX program
  → Ecologist partners provide MRV training + equipment

  Annual MRV:
  → eDNA: 892 species detected (baseline for health index)
  → Crab burrow counts: indicator of mangrove health
  → Fish biomass surveys: spillover to adjacent fisheries

  Tokenization:
  → 2,800 ha × 0.12 BSI improvement = 336 BDC/year
  → $50/BDC = $16,800/year directly to community fund
  → Community votes on fund allocation: school, clinic, patrol boat
```

### Use Case 4: Cross-Border Biodiversity Credit Trading

```
Kenyan grassland restoration project sells BDCs to European cosmetics company
  → No intermediary (saves 15–25% fees)
  → Settlement in ~5 seconds vs 90-day registry cycle
  → Full traceability: polygon #422, GPS boundary, eDNA survey hash
  → European company retires against TNFD nature-positive commitment
  → On-chain proof for EU CSRD / CS3D compliance
```

### Use Case 5: Habitat Banking for Infrastructure Offsets

```
Mining company required to offset 500 ha of woodland impact under UK BNG
  → Buys 750 BDC credits (150% compensation ratio required)
  → Retires against recipient polygon in same watershed
  → Polygon containment proof verifies ecological equivalence
  → Planning authority validates on-chain in minutes
  → 30-year management plan encoded in credit metadata
```

---

## 📜 Smart Contracts

### 1. BDC Token Contract

Implements **SEP-41** (Stellar Asset Contract) for a tokenized Biodiversity Credit.

| Property | Value |
|----------|-------|
| Standard | Stellar Asset Contract (SAC) / SEP-41 |
| Decimals | 0 (1 token = 1 verified biodiversity unit) |
| Fungibility | Semi-fungible: fungible within same biome + methodology class; non-fungible at individual credit level for polygon tracing |
| Metadata standard | Off-chain JSON on IPFS with content-addressed hash |
| Mint authority | Approval Governance contract only (cross-contract, after multi-stakeholder approval) |
| Burn authority | Any BDC holder + Retirement Registry |
| Transfer | Permissioned between non-retired tokens |
| Polygon binding | Each token references a `polygon_id` stored on-chain |

#### BDC Metadata Schema

```json
{
  "token_id": "bdc:stellar:mainnet:00000042",
  "credit": {
    "methodology": "BDCX-TF-v1.0",
    "biome": "tropical_forest",
    "country": "BR",
    "region": "Amazonas"
  },
  "polygon": {
    "id": "poly:br:am:0042",
    "area_ha": 1250,
    "coordinates_ipfs_hash": "QmX...",
    "bounding_box": {
      "min_lat": -3.42,
      "max_lat": -3.18,
      "min_lon": -60.12,
      "max_lon": -59.88
    }
  },
  "survey": {
    "ipfs_hash": "QmY...",
    "survey_date": "2026-06-04T10:00:00Z",
    "oracle_attestations": [
      "sig:edna-lab-01",
      "sig:cam-ai-03",
      "sig:sat-imagery-02",
      "sig:field-survey-07"
    ],
    "oracle_threshold": "4-of-7"
  },
  "biodiversity_metrics": {
    "baseline_bsi": 0.28,
    "current_bsi": 0.64,
    "delta_bsi": 0.36,
    "species_detected": 2400,
    "iucn_threatened_species": 3,
    "canopy_cover_pct": 38,
    "methodology_version": "v1.0"
  },
  "approval": {
    "governance_id": "gov:br:am:0042-001",
    "ecologist_approval": true,
    "community_approval": true,
    "auditor_approval": true,
    "approval_timestamp": "2026-06-10T14:30:00Z"
  },
  "vintage_year": 2026,
  "vintage_quarter": 2
}
```

#### BDC State Machine

```
         mint() (after multi-stakeholder approval)
  ┌──────────────────────────────────────────────┐
  │  Active                                       │
  │  (transferable + tradable)                    │
  │  bound to polygon_id                          │
  └──────┬───────────────────────────────────────┘
         │
         │ retire()
         ▼
  ┌──────────────────────────────────────────────┐
  │  Retired                                      │
  │  (frozen)                                     │
  │  polygon claim finalized                      │
  │  immutable receipt with geometry              │
  └──────────────────────────────────────────────┘
```

---

### 2. MRV Oracle Contract

Trust-minimized bridge between ecological survey data (field measurements, eDNA analysis, camera trap AI, satellite imagery) and on-chain biodiversity credits.

#### Oracle Network Topology

```
                          ┌─────────────────────────────┐
                          │  MRV Oracle Aggregator       │
                          │  (contract)                  │
                          └──────┬──────┬──────┬─────────┘
                                 │      │      │
                      ┌──────────┘  ┌───┘  ┌──┘
                      ▼              ▼      ▼
                ┌───────────┐  ┌───────────┐  ┌───────────┐
                │ eDNA Lab  │  │ Camera AI │  │ Satellite │  ... N
                │ (Staked)  │  │ (Staked)  │  │ (Staked)  │
                └───────────┘  └───────────┘  └───────────┘
                      │              │              │
                      ▼              ▼              ▼
                ┌────────────────────────────────────────┐
                │  IPFS (raw survey data + metadata)      │
                │  Content-addressed, encrypted at rest   │
                └────────────────────────────────────────┘
```

#### MRV Data Validation Flow

```
1. Field team collects survey data: eDNA samples, camera trap images, acoustic recordings, transect measurements
2. Data is hashed, encrypted, and uploaded to IPFS → returns CID
3. Metadata manifest created with survey methodology, equipment, GPS tracks
4. MRV oracle nodes independently fetch data from IPFS via CID
5. Each node runs its analysis pipeline:
   - eDNA Lab: sequences DNA, runs metabarcoding, generates species list
   - Camera AI: processes images, identifies species via ML model
   - Satellite: runs NDVI / canopy cover analysis on matched imagery window
   - Field Surveyor: validates ground-truth data, cross-checks with sensor data
6. Each node publishes an analysis hash + cross-validation signature
7. Oracle Handler contract validates N-of-M threshold signatures
8. Cross-validation passes only if ≥threshold independent analyses agree within tolerance
9. If dispute raised → ecologist panel reviews raw data; oracles slashed if fraudulent

Cross-validation tolerance:
  - Species detection: ≥80% overlap between independent methods
  - Canopy cover: ±5% between satellite and field measurement
  - BSI calculation: ±0.05 between any two oracle analyses
```

| Function | Description |
|----------|-------------|
| `register_oracle(pubkey, uri, oracle_type)` | Register oracle node (admin) with type (0=eDNA, 1=camera, 2=satellite, 3=field, 4=acoustic) |
| `revoke_oracle(pubkey)` | Remove oracle node (admin) |
| `set_threshold(n, d)` | Set N-of-M threshold (e.g. 3-of-5) |
| `set_cross_validation_config(min_overlap_pct)` | Set cross-validation tolerance parameters |
| `submit_survey(polygon_id, ipfs_cid, survey_timestamp, signatures[], analyses[])` | Submit validated survey with N attestations and analysis hashes |
| `verify_ipfs(ipfs_cid, expected_hash)` | Verify content-addressed data integrity |
| `dispute(survey_hash)` | Raise dispute (opens ecologist review window) |
| `resolve_dispute(survey_hash, outcome, slashed_oracles[])` | Resolve with slashing if fraud proven |
| `pause()` / `resume()` | Emergency halt |
| `set_polygon(polygon_id, geometry_ipfs_hash, bounding_box)` | Register habitat polygon with on-chain bounding box |
| `set_methodology(methodology_id, version, params)` | Register credit methodology |

---

### 3. Approval Governance Contract

Multi-stakeholder weighted voting engine that controls the minting of BDC tokens. This is the **core differentiator** of BDCX — no single actor can mint credits.

#### Stakeholder Roles & Weights

| Role | Weight | Description | Veto Power |
|------|--------|-------------|------------|
| **Lead Ecologist** | 3 | Certified field ecologist who designed the MRV protocol | No |
| **Peer Ecologist** | 2 | Independent ecologist reviewing the data | No |
| **Local Community Rep** | 2 | Representative of indigenous/local community | **Yes** |
| **Independent Auditor** | 1 | Third-party accredited auditor | No |
| **Methodology Expert** | 2 | Expert in the specific credit methodology | No |
| **Regulatory Observer** | 1 | Government or accreditation body rep | No veto, but single "no" triggers extended review |

**Approval threshold**: Weighted sum ≥ 6 (of 11 total possible)
**Community veto**: If community rep votes "no", proposal is automatically rejected regardless of total score.

#### Proposal & Voting Lifecycle

```
         propose()
  ┌──────────────────────────────────┐
  │  Draft                           │
  │  (submitted by project developer)│
  └──────────────┬───────────────────┘
                 │ voting period opens
                 ▼
  ┌──────────────────────────────────┐
  │  Voting                          │
  │  (stakeholders cast votes)       │
  │  Duration: configurable (7-30d)  │
  └──────┬──────────────┬────────────┘
         │              │
    threshold met    threshold not met
    + no veto        OR community veto
         ▼              ▼
  ┌──────────────┐  ┌──────────────────┐
  │ Approved     │  │ Rejected         │
  │ → mint()     │  │ (with reason)    │
  └──────────────┘  └──────────────────┘
```

| Function | Description |
|----------|-------------|
| `__constructor(admin, min_threshold, voting_period_secs)` | Initialize governance |
| `register_stakeholder(addr, role, weight, has_veto)` | Register stakeholder (admin) |
| `remove_stakeholder(addr)` | Remove (admin) |
| `set_threshold(min_weight)` | Set approval threshold |
| `set_voting_period(secs)` | Set voting duration |
| `propose(polygon_id, survey_hash, methodology_id, credit_qty, beneficiary)` | Create minting proposal |
| `vote(proposal_id, voter, approve: bool, comment_hash)` | Cast vote (stakeholder) |
| `veto(proposal_id, voter)` | Community veto (community role only) |
| `on_approved(proposal_id)` | Internal: cross-call to bdc-token.mint() |
| `get_proposal(proposal_id)` | Query proposal state |
| `get_vote(proposal_id, voter)` | Query individual vote |
| `get_stakeholder(addr)` | Query stakeholder info |
| `close_proposal(proposal_id)` | Admin finalize after voting period ends |

---

### 4. Retirement Registry Contract

Permanently removes BDCs from circulation with on-chain habitat polygon anchoring and verifiable proof for corporate TNFD/SBTN disclosure.

#### Retirement Certificate Schema

```json
{
  "receipt_id": "retire:bdc:stellar:mainnet:a1b2c3d4...",
  "retirement": {
    "token_ids": ["bdc:stellar:mainnet:00000042", "bdc:stellar:mainnet:00000043"],
    "total_credits": 250,
    "biome": "tropical_forest",
    "block_height": 3847291,
    "tx_hash": "0x..."
  },
  "claimer": {
    "stellar_address": "GABC...XYZ",
    "organization": "EcoTech Corp",
    "tnfd_id": "TNFD-2026-00422"
  },
  "polygon": {
    "id": "poly:br:am:0042",
    "area_ha": 1250,
    "bounding_box": {
      "min_lat": -3.42,
      "max_lat": -3.18,
      "min_lon": -60.12,
      "max_lon": -59.88
    },
    "geometry_ipfs_hash": "QmX..."
  },
  "claim": {
    "period_start": "2026-Q1",
    "period_end": "2026-Q2",
    "purpose": "TNFD nature-positive disclosure",
    "jurisdiction": "Global / SBTN",
    "standard": "BDCX-TF-v1.0"
  },
  "zero_knowledge_proof": {
    "merkle_root": "0x7eab...",
    "public_inputs_hash": "0x1f3a..."
  }
}
```

| Function | Description |
|----------|-------------|
| `retire(token_ids[], polygon_id, claim_data)` | Burn BDCs + record retirement with polygon anchoring |
| `get_retirement_receipt(receipt_id)` | Query full certificate with polygon data |
| `verify_retirement(token_id)` | Check if a specific BDC is retired |
| `verify_claim(claim_data, polygon_id)` | Verify entire claim against on-chain state |
| `prove_polygon_containment(token_id, polygon_id)` | Prove a retired credit was bound to a specific polygon |
| `prove_claim(wallet, period)` | Generate Merkle proof for portfolio claim (privacy-preserving) |
| `set_verifier(contract_id, authorized)` | Authorize external verifier contracts |
| `get_retired_polygons(claimer)` | List all polygons where claimer has retired credits |
| `set_geometry_verifier(contract_id)` | Authorize external geometry verification contract |

---

## 👥 Multi-Stakeholder Approval Workflow

This is the **heart of BDCX** — ensuring that no single interest can create or retire biodiversity credits without rigorous multi-party consent.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     APPROVAL WORKFLOW (DETAILED)                             │
│                                                                              │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐    │
│  │  Project    │   │  Ecologist  │   │  Community  │   │  Auditor    │    │
│  │  Developer  │   │  Panel (3) │   │  Rep (1)    │   │  (1)        │    │
│  └──────┬──────┘   └──────┬──────┘   └──────┬──────┘   └──────┬──────┘    │
│         │                  │                  │                  │           │
│  1. Submit                │                  │                  │           │
│  proposal ───────────────►│                  │                  │           │
│         │                 │                  │                  │           │
│         │           2. Review MRV data       │                  │           │
│         │           from IPFS hash           │                  │           │
│         │◄─────────────────┐                 │                  │           │
│         │                 │                  │                  │           │
│         │           3. Vote approve/reject   │                  │           │
│         │◄──────────────────────────────────►│◄─────────────────│           │
│         │                 │                  │                  │           │
│         │                 │  4. If community says NO →         │           │
│         │                 │  AUTOMATIC VETO (cannot override)   │           │
│         │                 │                  │                  │           │
│         │           5. Threshold check: weighted sum >= 6      │           │
│         │                 │                  │                  │           │
│         │◄──────────── ALL APPROVED ──────────────────────────►│           │
│         │                 │                  │                  │           │
│  6. BDC minted           │                  │                  │           │
│  ◄──────────────────────────────────────────────────────────────│           │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Voting Weights & Thresholds

| Composition | Weight | Count | Total |
|-------------|--------|-------|-------|
| Lead Ecologist | 3 | 1 | 3 |
| Peer Ecologist | 2 | 1 | 2 |
| Community Rep | 2 | 1 | 2 |
| Independent Auditor | 1 | 1 | 1 |
| Methodology Expert | 2 | 1 | 2 |
| Regulatory Observer | 1 | 1 | 1 |
| **Total** | | **6** | **11** |

**Approval**: Weighted sum ≥ 6 AND no community veto
**Rejection triggers**: Community veto, weighted sum < 6, any "fraud" flag from auditor

### Conflict Resolution

If a proposal is contested:
1. **Mediation period**: 14-day extension for additional data submission
2. **Expert panel**: 3-person panel drawn from methodology expert pool
3. **Appeal**: Project developer can appeal to full stakeholder assembly (all stakeholders vote with equal weight)
4. **Arbitration**: Final binding arbitration by accredited body (configurable per jurisdiction)

---

## 🔬 MRV Data Model & IPFS Integration

### Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              OFF-CHAIN                                            │
│                                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │  Survey Data Package (uploaded to IPFS)                                     │ │
│  │                                                                              │ │
│  │  ├── manifest.json         (metadata, methodology, equipment list)           │ │
│  │  ├── edna/                  (raw FASTQ sequences + metabarcoding results)    │ │
│  │  │   ├── sample_001.fastq.gz                                                │ │
│  │  │   ├── sample_002.fastq.gz                                                │ │
│  │  │   ├── taxonomy.csv       (species abundance table)                       │ │
│  │  │   └── pcr_protocol.json                                                  │ │
│  │  ├── camera_traps/          (images + AI inference results)                  │ │
│  │  │   ├── cam_01_images/                                                      │ │
│  │  │   ├── cam_02_images/                                                      │ │
│  │  │   ├── species_detections.csv  (MegaDetector / WildMe output)             │ │
│  │  │   └── activity_patterns.json                                              │ │
│  │  ├── acoustics/             (audio recordings + analysis)                    │ │
│  │  │   ├── recordings/                                                         │ │
│  │  │   ├── birdnet_results.csv  (BirdNET/AudioSet inference)                  │ │
│  │  │   └── acoustic_indices.json                                               │ │
│  │  ├── satellite/             (satellite imagery indices)                      │ │
│  │  │   ├── ndvi_timeseries.csv                                                 │ │
│  │  │   ├── canopy_cover_analysis.json                                          │ │
│  │  │   └── land_cover_classification.tif                                       │ │
│  │  ├── field_transects/       (ground-truth surveys)                           │ │
│  │  │   ├── vegetation_plots.csv                                                │ │
│  │  │   ├── soil_samples.csv                                                    │ │
│  │  │   └── pollinator_observations.csv                                         │ │
│  │  └── gps_tracks/            (GPS traces of field work)                       │ │
│  │      └── survey_tracks.gpx                                                   │ │
│  └─────────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                               │
│                                    ▼                                               │
│                    Content-addressed IPFS hash (CID)                               │
│                    Example: QmX5P2r5K8Qr3N1Qz7Tz9Y...                              │
│                                    │                                               │
└────────────────────────────────────┼───────────────────────────────────────────────┘
                                     │ CID submitted on-chain
                                     ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              ON-CHAIN                                            │
│                                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────────────┐ │
│  │  MRV Oracle Handler stores:                                                   │ │
│  │  ──────────────────────────                                                   │ │
│  │  polygon_id → (geometry_ipfs_cid, bounding_box, last_survey_cid)              │ │
│  │  ipfs_cid → (survey_hash, oracle_sigs[], status, disputed, token_id)          │ │
│  │  oracle_pubkey → (type, uri, active, stake, total_surveys, accuracy_score)   │ │
│  └─────────────────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### IPFS Content Addressing

Each survey data package is hashed using the IPFS content-addressing scheme. The resulting CID (Content Identifier) is the only data stored on-chain — raw survey data remains off-chain for scalability and privacy.

```
survey_data_cid = ipfs_add_directory({
    "manifest.json": manifest_bytes,
    "edna/": edna_directory,
    "camera_traps/": camera_directory,
    ...
})

// Only the CID is submitted to the MRV oracle contract
mrv_oracle.submit_survey(polygon_id, survey_data_cid, survey_timestamp, signatures, analyses)
```

### Oracle Analysis Hash Schema

Each oracle node independently analyzes the raw data and produces a structured analysis result:

```json
{
  "oracle_id": "edna-lab-01",
  "survey_cid": "QmX5P2r5K8Qr3N1Qz7Tz9Y...",
  "analysis_hash": "0xabc123...",
  "metrics": {
    "species_richness": 342,
    "shannon_diversity": 4.2,
    "iucn_threatened_count": 3,
    "bsi_contribution": 0.18,
    "confidence_score": 0.94
  },
  "cross_validation": {
    "agreement_with_satellite": 0.88,
    "agreement_with_camera": 0.92,
    "agreement_with_field": 0.85
  },
  "timestamp": 1717500000,
  "signature": "0x..."
}
```

---

## 📐 Habitat Polygon System

### On-Chain Polygon Representation

Full GPS polygon geometry is stored **off-chain on IPFS**. On-chain, we store a **bounding box** and the **IPFS CID** of the full geometry. This balances on-chain verifiability with gas efficiency.

```rust
pub struct PolygonGeometry {
    pub polygon_id: BytesN<32>,         // hash derived from GPS coordinates + project
    pub geometry_ipfs_cid: Bytes,       // IPFS CID of full GeoJSON polygon
    pub bounding_box: BoundingBox,      // min/max lat/lon for quick containment checks
    pub area_ha: u64,                   // total habitat area in hectares
    pub biome: u8,                      // 0=tropical_forest, 1=temperate_forest, 2=grassland,
                                        // 3=wetland, 4=mangrove, 5=coral_reef, 6=other
    pub country: BytesN<2>,             // ISO 3166-1 alpha-2
    pub project_id: BytesN<32>,         // link to project metadata
    pub registered_at: u64,             // timestamp
    pub active: bool,                   // polygon can accept new BDC minting
}
```

### Point-in-Polygon Containment

The retirement contract implements a simple **ray-casting algorithm** for point-in-polygon containment checks:

```rust
pub fn point_in_polygon(
    point: (i64, i64),          // (lat * 1_000_000, lon * 1_000_000) fixed-point
    polygon: Vec<(i64, i64)>,   // vertices in fixed-point
) -> bool;
```

This allows verifiers to prove that a retired credit's polygon contains a specific GPS coordinate, without revealing the full polygon geometry.

### Polygon Lifecycle

```
         register_polygon()
  ┌──────────────────────────────────┐
  │  Registered                      │
  │  (active = true)                 │
  │  ready for MRV surveys           │
  └──────────────┬───────────────────┘
                 │
          survey submitted
          credits minted
                 ▼
  ┌──────────────────────────────────┐
  │  Active                          │
  │  (accumulates BDC credits)       │
  │  credits reference polygon_id    │
  └──────┬───────────────────────────┘
         │
    polygon retired (all credits
    consumed) OR admin action
         ▼
  ┌──────────────────────────────────┐
  │  Closed                          │
  │  (active = false)                │
  │  historical record only          │
  └──────────────────────────────────┘
```

---

## 📊 Credit Methodology

### Biodiversity Score Index (BSI)

The BSI is a composite metric (0.0–1.0) that quantifies the ecological health of a habitat polygon. It is computed **off-chain** by oracle nodes and verified through cross-validation.

```
BSI = 0.30 × SR_norm  +  0.25 × SDI_norm  +  0.20 × CC_norm  +  0.15 × IUCN_norm  +  0.10 × FQ_norm

Where:
  SR_norm    = Species Richness / max_expected (normalized 0–1)
  SDI_norm   = Shannon Diversity Index / ln(max_expected) (normalized 0–1)
  CC_norm    = Canopy Cover % / 100
  IUCN_norm  = 1 - (threatened_species_ratio / 0.5) [capped at 0]
  FQ_norm    = Floristic Quality Index / max_possible
```

### Credit Calculation

```
BDC_credits = (BSI_current - BSI_baseline) × polygon_area_ha × quality_multiplier

Where:
  quality_multiplier = 1.0 (standard)
                     1.2 (for IUCN priority habitats)
                     1.5 (for indigenous/managed areas with FPIC)
                     0.8 (for restored vs. preserved)
```

### Methodology Versions

| Version | Status | Description |
|---------|--------|-------------|
| BDCX-TF-v1.0 | Active | Tropical Forest — eDNA + camera traps + satellite |
| BDCX-MG-v1.0 | Active | Mangrove — eDNA + crab burrows + sediment cores |
| BDCX-GS-v1.0 | Beta | Grassland/Savanna — transects + acoustic + satellite |
| BDCX-WL-v1.0 | Draft | Wetland — hydrology + eDNA + bird surveys |

---

## 💰 Tokenomics & Fee Model

### Protocol Fees

| Fee Type | Rate | Payer | Recipient |
|----------|------|-------|-----------|
| Spot trading fee | 0.25% (25 bps) | Both sides (taker) | Protocol treasury |
| CfD opening fee | 0.10% (10 bps) of notional | Both parties | Protocol treasury |
| CfD settlement fee | $10 flat per position | Party paying difference | Protocol treasury |
| MRV oracle attestation fee | $5 per survey | Project developer (deducted at mint) | Oracle node operators |
| BDC retirement fee | $5 flat per batch | Retirer | Protocol treasury |
| Market maker rebate | −0.05% (−5 bps) | Protocol → maker | Maker side |
| Methodology registration | $500 flat | Project developer | Protocol treasury |
| Polygon registration | $100 flat | Project developer | Protocol treasury |

### Fee Distribution

```
Protocol Fees Collected
        │
        ▼
  ┌──────────────────────────────────────┐
  │  40% → Protocol Treasury             │ ← governance-controlled spending
  │  25% → MRV Oracle Pool               │ ← distributed to honest oracle nodes
  │  15% → Community Benefit Pool        │ ← directed to local communities
  │  10% → Ecologist/Verifier Pool       │ ← distributed to approving ecologists
  │  10% → Ecosystem Grants              │ ← methodology dev, research, tools
  └──────────────────────────────────────┘
```

### Inflation & Supply Caps

- No protocol token inflation (BDCX uses yUSDC as quote currency)
- BDC supply = total verified biodiversity units (tied to real-world ecological outcomes)
- Protocol sustainability via fee revenue, not token emissions
- **Hard cap**: Maximum 1 billion BDC tokens globally (configurable by governance)

---

## 🗃 Data Model

### BDC Token

```rust
pub struct BdcToken {
    pub token_id: u64,                // auto-increment
    pub polygon_id: BytesN<32>,       // link to habitat polygon
    pub methodology_id: BytesN<8>,    // methodology version identifier
    pub survey_ipfs_cid: Bytes,       // IPFS CID of generating survey
    pub baseline_bsi: u32,            // BSI * 100 (e.g. 28 = 0.28)
    pub current_bsi: u32,             // BSI * 100 (e.g. 64 = 0.64)
    pub area_ha_contribution: u64,    // ha represented by this credit
    pub biome: u8,                    // biome type
    pub vintage_year: u16,
    pub vintage_quarter: u8,
    pub approval_governance_id: BytesN<32>,
    pub owner: Address,
    pub state: u8,                    // 0=active, 1=retired
    pub retired_at: Option<u64>,
    pub retirement_receipt: Option<BytesN<32>>,
}
```

### Survey Record

```rust
pub struct SurveyRecord {
    pub survey_hash: BytesN<32>,       // SHA256 of IPFS CID + timestamp + polygon
    pub polygon_id: BytesN<32>,
    pub ipfs_cid: Bytes,               // IPFS content identifier
    pub survey_timestamp: u64,
    pub oracle_count: u32,
    pub threshold_met: bool,
    pub disputed: bool,
    pub resolved: bool,
    pub token_ids: Vec<u64>,           // BDC tokens minted from this survey
    pub analyses_hashes: Vec<BytesN<32>>,  // individual oracle analysis hashes
}
```

### Proposal (Governance)

```rust
pub struct Proposal {
    pub proposal_id: u64,
    pub polygon_id: BytesN<32>,
    pub survey_hash: BytesN<32>,
    pub methodology_id: BytesN<8>,
    pub credit_qty: u64,
    pub beneficiary: Address,
    pub proposer: Address,
    pub created_at: u64,
    pub voting_deadline: u64,
    pub state: u8,                      // 0=draft, 1=voting, 2=approved, 3=rejected, 4=cancelled
    pub votes: Vec<Vote>,
    pub community_veto: bool,
    pub weighted_total_approve: u32,
    pub weighted_total_reject: u32,
}
```

### Habitat Polygon

```rust
pub struct HabitatPolygon {
    pub polygon_id: BytesN<32>,
    pub geometry_ipfs_cid: Bytes,       // full GeoJSON on IPFS
    pub bounding_box: BoundingBox,
    pub area_ha: u64,
    pub biome: u8,
    pub country: BytesN<2>,
    pub project_id: BytesN<32>,
    pub registered_at: u64,
    pub active: bool,
    pub total_credits_minted: u64,
    pub total_credits_retired: u64,
    pub last_survey_cid: Option<Bytes>,
    pub last_survey_timestamp: Option<u64>,
}

pub struct BoundingBox {
    pub min_lat: i64,     // latitude * 1_000_000
    pub max_lat: i64,
    pub min_lon: i64,     // longitude * 1_000_000
    pub max_lon: i64,
}
```

### Order

```rust
pub struct Order {
    pub order_id: u64,
    pub trader: Address,
    pub side: u8,              // 0=buy, 1=sell
    pub price: i128,           // in yUSDC (7 decimals)
    pub initial_qty: u64,      // BDC credits
    pub remaining_qty: u64,
    pub timestamp: u64,
    pub restrictions: u8,      // 0=none, 1=FOK, 2=IOC
    pub biome_filter: Option<u8>,
    pub vintage_filter: Option<u16>,
    pub status: u8,            // 0=open, 1=filled, 2=cancelled
}
```

### CfD Position

```rust
pub struct CfDPosition {
    pub position_id: u64,
    pub counterparty_a: Address,    // long / project developer
    pub counterparty_b: Address,    // short / corporate buyer
    pub strike_price: i128,         // yUSDC per BDC
    pub quantity: u64,              // BDCs
    pub settlement_date: u64,
    pub collateral_a: i128,
    pub collateral_b: i128,
    pub maintenance_margin_bps: u16,
    pub oracle_feed_id: BytesN<8>,
    pub biome: u8,
    pub state: u8,                  // 0=pending, 1=active, 2=settled, 3=expired, 4=liquidated
    pub last_mtm_timestamp: u64,
    pub mtm_value: i128,
}
```

### Retirement Receipt

```rust
pub struct RetirementReceipt {
    pub receipt_id: BytesN<32>,
    pub retirer: Address,
    pub token_ids: Vec<u64>,
    pub polygon_id: BytesN<32>,
    pub total_credits: u64,
    pub claim_period_start: u64,
    pub claim_period_end: u64,
    pub purpose: Bytes,
    pub jurisdiction: Bytes,
    pub merkle_root: BytesN<32>,
    pub timestamp: u64,
    pub block_height: u64,
}
```

---

## 📡 Event & Error Reference

### Events (all contracts emit these)

| Event | Emitted By | Data |
|-------|-----------|------|
| `BdcMinted(token_id, polygon_id, survey_hash, qty)` | BDC Token | Mint events |
| `BdcTransferred(token_id, from, to)` | BDC Token | Transfers |
| `BdcBurned(token_id, retirer)` | BDC Token | Burns |
| `SurveySubmitted(survey_hash, polygon_id, ipfs_cid, oracle_count)` | MRV Oracle | New survey |
| `SurveyValidated(survey_hash, polygon_id, analysis_hashes[])` | MRV Oracle | Validation passed |
| `SurveyDisputed(survey_hash, disputer)` | MRV Oracle | Dispute raised |
| `SurveyResolved(survey_hash, outcome, slashed[])` | MRV Oracle | Resolution |
| `OracleRegistered(pubkey, oracle_type)` | MRV Oracle | New oracle |
| `OracleRevoked(pubkey)` | MRV Oracle | Oracle removed |
| `PolygonRegistered(polygon_id, biome, area_ha)` | MRV Oracle | New habitat polygon |
| `PolygonClosed(polygon_id)` | MRV Oracle | Polygon deactivated |
| `ProposalCreated(proposal_id, polygon_id, credit_qty)` | Approval Gov | New mint proposal |
| `VoteCast(proposal_id, voter, approve, weight)` | Approval Gov | Vote recorded |
| `CommunityVeto(proposal_id, voter)` | Approval Gov | Veto exercised |
| `ProposalApproved(proposal_id, token_ids[])` | Approval Gov | Minting authorized |
| `ProposalRejected(proposal_id, reason)` | Approval Gov | Proposal denied |
| `StakeholderRegistered(addr, role, weight)` | Approval Gov | New stakeholder |
| `StakeholderRemoved(addr)` | Approval Gov | Stakeholder removed |
| `OrderPlaced(order_id, trader, side, price, qty)` | Marketplace | New order |
| `OrderCancelled(order_id)` | Marketplace | Cancel |
| `OrderFilled(order_id, fill_qty, fill_price, fee)` | Marketplace | Fill |
| `OrderMatched(buy_id, sell_id, qty, price, fee)` | Marketplace | Match event |
| `CfDOpened(position_id, a, b, strike, qty, biome, expiry)` | Marketplace | New CfD |
| `CfDSettled(position_id, spot_price, net_transfer)` | Marketplace | CfD close |
| `CfDLiquidated(position_id, losing_party)` | Marketplace | Forced close |
| `MarginCall(position_id, shortfall)` | Marketplace | Collateral alert |
| `BdcRetired(receipt_id, retirer, polygon_id, token_count, total_credits)` | Retirement | Retirement |
| `ClaimVerified(claim_id, polygon_id, valid)` | Retirement | Verification |
| `ContractPaused(contract_id)` | All (pauseable) | Emergency |
| `ContractResumed(contract_id)` | All (pauseable) | Resume |
| `AdminTransferred(old_admin, new_admin)` | All | Ownership change |

### Error Codes

| Code | Name | Description |
|------|------|-------------|
| `E001` | `Unauthorized` | Caller lacks required role |
| `E002` | `BdcAlreadyRetired` | Token already burned |
| `E003` | `InsufficientBalance` | Not enough BDCs |
| `E004` | `TokenNotFound` | Token ID doesn't exist |
| `E005` | `PolygonNotFound` | Polygon ID not registered |
| `E006` | `PolygonInactive` | Polygon is closed |
| `E007` | `SurveyNotFound` | Survey hash not found |
| `E008` | `SurveyAlreadyResolved` | Dispute already resolved |
| `E009` | `InvalidOracleSignature` | Sig doesn't match registered oracle |
| `E010` | `OracleThresholdNotMet` | Not enough sigs |
| `E011` | `InvalidSurveyData` | Plausibility/validation check failed |
| `E012` | `ContractPaused` | Operation not allowed while paused |
| `E013` | `ProposalNotFound` | Proposal ID doesn't exist |
| `E014` | `ProposalNotVoting` | Not in voting state |
| `E015` | `ProposalAlreadyClosed` | Already approved/rejected |
| `E016` | `VoterNotStakeholder` | Not registered as stakeholder |
| `E017` | `VoteAlreadyCast` | Already voted |
| `E018` | `CommunityVetoActivated` | Veto exercised, cannot approve |
| `E019` | `ThresholdNotMet` | Weighted sum below min |
| `E020` | `OrderNotFound` | Order ID doesn't exist |
| `E021` | `OrderFullyFilled` | No remaining quantity |
| `E022` | `PriceMismatch` | Buy price < sell price |
| `E023` | `InsufficientCollateral` | CfD posting too low |
| `E024` | `CollateralBelowMaintenance` | Margin call triggered |
| `E025` | `CfDNotSettled` | Position still active |
| `E026` | `CfDAlreadySettled` | Position already closed |
| `E027` | `BiomeMismatch` | BDC biome ≠ order filter |
| `E028` | `ArithmeticOverflow` | Safe math failure |
| `E029` | `FeeCapExceeded` | Fee rate above governance max |
| `E030` | `DuplicateSurvey` | Survey hash already submitted |
| `E031` | `StakeholderAlreadyRegistered` | Already a stakeholder |
| `E032` | `StakeholderNotFound` | Not registered |
| `E033` | `ReceiptNotFound` | Retirement receipt not found |

---

## 🔀 Cross-Contract Interaction Flow

### Full Credit Lifecycle (Detailed Sequence)

```
Field        MRV Oracle Node 1   MRV Oracle Node 2   Approval Gov       BDC Token       Retirement
Team             │                     │                  │                 │                │
 │               │                     │                  │                 │                │
 │──IPFS────────►│                     │                  │                 │                │
 │──CID─────────►│─────verify(CID)────►│                  │                 │                │
 │               │──sign(hash)──┐      │                  │                 │                │
 │               │              │      │                  │                 │                │
 │               │              │      │──verify(CID)────►│                 │                │
 │               │              │      │──sign(hash)──┐   │                 │                │
 │               │              │      │              │   │                 │                │
 │               │◄────submit_survey(polygon_id, CID, sigs[2], analyses)───────────────►    │
 │               │              │                     │   │                 │                │
 │               │              │                     │   │──validate_sigs(3-of-5)          │
 │               │              │                     │   │──cross_validation               │
 │               │              │                     │   │─►emit SurveyValidated            │
 │               │              │                     │   │                 │                │
 │               │              │                     │   │──propose()──────►               │
 │               │              │                     │   │                 │                │
 │               │              │                     │   │──vote(approve)──►               │
 │               │              │                     │   │──vote(approve)──►               │
 │               │              │                     │   │──vote(approve)──►               │
 │               │              │                     │   │                 │                │
 │               │              │                     │   │──threshold_met──►               │
 │               │              │                     │   │                 │                │
 │               │              │                     │   │────mint()───────►──────────────►│
 │               │              │                     │   │                 │                │
 │               │              │                     │   │                 │──create_token()│
 │               │              │                     │   │                 │──emit BdcMinted│
 │               │              │                     │   │                 │                │
 │               │              │                     │   │◄─token_ids──────┤                │
 │               │              │                     │   │                 │                │
 │◄═══════════════════════════════════════════════════════════════════════════════════════════
```

### Retirement Flow (Detailed Sequence)

```
Retirement Contract    BDC Token Contract       Retirer (Corporate Buyer)
       │                       │                         │
       │                       │                         │
       │◄──────retire(token_ids[], polygon_id, claim_data)───│
       │                       │                         │
       │──for each token_id:   │                         │
       │──burn(token_id)──────►│                         │
       │◄──────BdcBurned───────│                         │
       │                       │                         │
       │──verify polygon_id    │                         │
       │   matches each token  │                         │
       │                       │                         │
       │──compute_merkle_root()│                         │
       │──generate_receipt()   │                         │
       │──emit BdcRetired      │                         │
       │                       │                         │
       │◄──receipt_id──────────│─────────────────────────│
       │                       │                         │
       │──verify_claim(claim_id)──►(external auditor)    │
       │   returns polygon_id, receipt, merkle proof     │
```

---

## 🏛 Governance Model

### Permission Model

| Role | Can | Assigned By |
|------|-----|-------------|
| **Admin** | Deploy contracts, transfer admin, pause/resume, set fees, upgrade | Initial deployer; transferable |
| **Oracle Node** | Submit survey attestations, register polygons | Admin |
| **Stakeholder (Ecologist)** | Vote on proposals, review IPFS data | Admin (governance contract) |
| **Stakeholder (Community)** | Vote on proposals, exercise veto | Admin (governance contract) |
| **Stakeholder (Auditor)** | Vote on proposals, flag fraud | Admin (governance contract) |
| **Project Developer** | Submit proposals, register polygons | Any address (permissioned by admin) |
| **Market Maker** | Match orders at preferred rate, access bulk matching | Admin |
| **Retirer** | Burn BDCs, generate certificates | Any address (permissionless) |
| **Trader** | Place/cancel orders, open CfD positions | Any address |
| **Auditor** | Query any state, verify retirement | Any address (read-only) |

### Future Governance (Phase 6+)

- Transition to **DAO-controlled** parameters via Soroban token-based voting
- Parameters subject to governance:
  - Fee rates (within hard-coded bounds)
  - Oracle threshold configuration
  - Stakeholder weights and approval thresholds
  - Maintenance margin ratios
  - Treasury allocation
  - Methodology version approvals
  - Contract upgrades
  - Community benefit pool distribution

---

## ⚖ Regulatory & Standards Alignment

### Biodiversity Standards Compliance

| Standard | Compliance | Notes |
|----------|-----------|-------|
| IUCN Global Standard for NbS | ✓ | Polygons mapped to IUCN ecosystem typology |
| TNFD (Taskforce on Nature-related Financial Disclosures) | ✓ | Retirement receipts structured for TNFD LEAP approach |
| SBTN (Science-Based Targets for Nature) | ✓ | Credit vintages aligned with SBTN 5-year cycles |
| Plan Vivo | TBD | Community-focused carbon+biodiversity co-benefits |
| Verra SD VISta | TBD | Sustainable Development Verified Impact Standard |
| UK BNG (Biodiversity Net Gain) | ✓ | Polygon geometry compatible with Statutory Biodiversity Metric |
| EU CSRD / ESRS E4 | ✓ | Retirement data maps to ESRS E4 disclosure requirements |
| GRI 304 (Biodiversity) | ✓ | On-chain audit trail satisfies GRI 304-1 to 304-4 |
| CBD Kunming-Montreal Framework | ✓ | Aligned with Target 19 (resource mobilization) |

### Anti-Fraud & Integrity

- **Survey tampering**: Multiple oracle attestations + cross-validation (±5% tolerance between methods)
- **Double minting**: `polygon_id + survey_timestamp` uniqueness constraint enforced by contract
- **Location fraud**: GPS-tracked field surveys + satellite imagery cross-reference
- **Vintage fraud**: BDC creation block timestamp compared to claimed survey time (rejects future-dated credits)
- **Oracle collusion**: N-of-M threshold model with slashing; economic disincentive for false reporting
- **Greenwashing**: Multi-stakeholder approval ensures no single actor can create unverified credits

### Jurisdictional Considerations

- Each polygon carries `country` and `biome` fields
- Retirement contract validates that claim jurisdiction matches polygon jurisdiction
- Methodology versions can be jurisdiction-specific (e.g., BDCX-AMZ-v1.0 for Amazon-specific protocols)
- Benefit-sharing agreements can be encoded in polygon metadata (e.g., 20% of credit sales to community fund)

---

## 🛠 Technical Stack

| Layer | Technology | Rationale |
|-------|-----------|-----------|
| **Blockchain** | Stellar (Soroban) | Low fees ($0.00001), fast finality (3–5s), carbon-friendly, built-in DEX primitive |
| **Smart Contracts** | Rust + Soroban SDK v22.x | Type-safe, WASM-compiled, deterministic execution |
| **Token Standard** | SEP-41 (Stellar Asset Contract) | Interoperable with Stellar ecosystem wallets/exchanges |
| **Oracle Network** | Custom Rust-based MRV oracle with threshold BLS signatures | Multi-modal ecological data, cross-validation, slashing-enabled |
| **Data Storage** | IPFS / Filecoin | Content-addressed, decentralized, permanent survey data storage |
| **Frontend** | React 19 + `@stellar/stellar-sdk` + MapLibre GL JS | Geo-spatial dashboard with polygon visualization |
| **Indexer** | Stellar RPC + custom sink to PostgreSQL | Real-time event stream for dashboards and analytics |
| **GIS Integration** | GeoJSON → on-chain bounding boxes | Efficient on-chain geometry without full polygon storage |
| **Testing** | `cargo test` · Soroban sandbox · `cargo expand` · fuzz testing | Full coverage including edge cases |

---

## 🔍 Comparison With Existing Solutions

| Feature | BDCX | Verra (VCU) | Plan Vivo | Regen Network | Toucan |
|---------|------|-------------|-----------|---------------|--------|
| **Asset type** | Biodiversity credits | Carbon credits | Carbon+biodiversity | Carbon+biodiversity | Carbon credits |
| **Blockchain** | Stellar (Soroban) | None (off-chain) | None (off-chain) | Cosmos (Regen Ledger) | Celo / Polygon |
| **MRV type** | Multi-modal (eDNA+traps+sat+acoustic) | Manual desk review | Community-based | Satellite + field | Desk review |
| **Oracle threshold** | N-of-M cross-validation | Single auditor | Single verifier | Single oracle | Single attestor |
| **Multi-stakeholder** | ✅ Ecologists + Community + Auditors | ❌ Single verifier | ❌ Single verifier | ❌ Single validator | ❌ Single bridge |
| **Community veto** | ✅ On-chain | ❌ | ✅ (off-chain) | ❌ | ❌ |
| **Polygon anchoring** | ✅ On-chain bounding box | ❌ Not tracked | ❌ Not tracked | ❌ Not tracked | ❌ Not tracked |
| **Plolygon retirement** | ✅ Per-polygon retirement claim | ❌ Not possible | ❌ Not possible | ❌ Not possible | ❌ Not possible |
| **IPFS data linkage** | ✅ Survey data on IPFS | ❌ PDF on registry | ❌ PDF on registry | ✅ On-chain | ❌ Centralized |
| **Settlement latency** | ~5 seconds | 30–90 days | 60–180 days | ~5 seconds | ~15 seconds |
| **Secondary market** | ✅ On-chain order book | ❌ OTC only | ❌ OTC only | ✅ AMM | ✅ AMM |
| **Protocol fees** | 0.25% | 3–15% broker | 5–15% | 0.1% | 0.1% |
| **TNFD/SBTN aligned** | ✅ By design | ❌ | ❌ | ❌ | ❌ |

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.81+ (`rustup update stable`)
- Soroban CLI v22.x
- Node.js 20+ (for frontend)
- IPFS daemon (or Kubo CLI) for testing MRV data pipeline

### Quick Start

```bash
# Clone and build
git clone https://github.com/your-org/biodiversity-credit-exchange
cd biodiversity-credit-exchange
cargo build --release

# Run all tests
cargo test --all-features

# Check formatting and linting
cargo fmt --check
cargo clippy --all-targets -- -D warnings

# Deploy to local sandbox
./scripts/setup.sh

# Deploy to testnet
./scripts/deploy.sh testnet

# Start frontend
cd frontend
npm install
npm run dev
```

### Project Scripts

| Script | Description |
|--------|-------------|
| `scripts/setup.sh` | Local sandbox bootstrap (install Soroban CLI, build, deploy to local) |
| `scripts/deploy.sh [network]` | Testnet/mainnet deployment (build, deploy, init contracts) |
| `scripts/test.sh` | Run all tests with coverage + linters |

---

## 📖 API Reference

### BDC Token Contract

| Function | Signature | Description |
|----------|-----------|-------------|
| `__constructor` | `(admin: Address)` | Initialize with admin |
| `admin` | `() -> Address` | Get admin address |
| `transfer_admin` | `(new_admin: Address)` | Transfer admin |
| `total_supply` | `() -> u64` | Total BDCs minted |
| `balance_of` | `(owner: Address) -> u64` | BDC balance |
| `owner_of` | `(token_id: u64) -> Address` | Token owner |
| `token_uri` | `(token_id: u64) -> Bytes` | IPFS metadata URI |
| `token_metadata` | `(token_id: u64) -> BdcMetadata` | Full token metadata |
| `mint` | `(to, polygon_id, methodology_id, survey_hash, baseline_bsi, current_bsi, area_contribution, biome, vintage_year, vintage_qtr, governance_id) -> u64` | Mint BDC (approval-gov only) |
| `transfer` | `(from, to, token_id)` | Transfer BDC |
| `burn` | `(caller, token_id)` | Burn BDC |
| `authorize_minter` | `(minter: Address)` | Set minter (admin) |
| `authorize_burner` | `(burner: Address)` | Set burner (admin) |
| `revoke_minter` | `()` | Remove minter |
| `revoke_burner` | `()` | Remove burner |
| `tokens_by_owner` | `(owner, start, limit) -> Vec<u64>` | Paginated token list |
| `tokens_by_polygon` | `(polygon_id, start, limit) -> Vec<u64>` | Tokens for a polygon |
| `set_metadata_uri` | `(token_id, new_uri)` | Update metadata URI (admin) |

### MRV Oracle Contract

| Function | Signature | Description |
|----------|-----------|-------------|
| `__constructor` | `(admin, rec_token_addr, default_threshold_n, default_threshold_d)` | Initialize |
| `admin` | `() -> Address` | Get admin |
| `transfer_admin` | `(new_admin)` | Admin transfer |
| `set_rec_token` | `(addr)` | Set BDC token contract |
| `register_oracle` | `(pubkey, uri, oracle_type)` | Register oracle (admin) |
| `revoke_oracle` | `(pubkey)` | Revoke oracle (admin) |
| `register_polygon` | `(polygon_id, geometry_ipfs_cid, bounding_box, area_ha, biome, country, project_id)` | Register polygon (admin) |
| `close_polygon` | `(polygon_id)` | Deactivate polygon |
| `get_polygon` | `(polygon_id) -> HabitatPolygon` | Query polygon |
| `submit_survey` | `(polygon_id, ipfs_cid, survey_timestamp, signatures[], analyses[]) -> u64` | Submit MRV survey |
| `dispute` | `(survey_hash)` | Raise dispute |
| `resolve_dispute` | `(survey_hash, outcome, slashed_oracles[])` | Resolve dispute (admin) |
| `set_threshold` | `(n, d)` | Set N-of-M threshold (admin) |
| `pause` / `resume` | `()` | Emergency controls |
| `oracle_count` | `() -> u32` | Registered oracle count |

### Approval Governance Contract

| Function | Signature | Description |
|----------|-----------|-------------|
| `__constructor` | `(admin, rec_token_addr, min_weight, voting_period_secs)` | Initialize |
| `register_stakeholder` | `(addr, role, weight, has_veto)` | Register (admin) |
| `remove_stakeholder` | `(addr)` | Remove (admin) |
| `set_threshold` | `(min_weight)` | Set approval threshold |
| `set_voting_period` | `(secs)` | Set voting duration |
| `propose` | `(polygon_id, survey_hash, methodology_id, credit_qty, beneficiary) -> u64` | Create proposal |
| `vote` | `(proposal_id, approve, comment_hash)` | Cast vote |
| `veto` | `(proposal_id)` | Community veto |
| `get_proposal` | `(proposal_id) -> Proposal` | Query proposal |
| `get_vote` | `(proposal_id, voter) -> Vote` | Query vote |
| `get_stakeholder` | `(addr) -> Stakeholder` | Query stakeholder |
| `close_proposal` | `(proposal_id)` | Force close after deadline |

### Retirement Registry Contract

| Function | Signature | Description |
|----------|-----------|-------------|
| `__constructor` | `(admin, rec_token_addr)` | Initialize |
| `set_rec_token` | `(addr)` | Set BDC token (admin) |
| `retire` | `(token_ids[], polygon_id, claim_data) -> BytesN<32>` | Retire BDCs |
| `get_receipt` | `(receipt_id) -> RetirementReceipt` | Query receipt |
| `verify_retirement` | `(token_id) -> bool` | Check if retired |
| `verify_claim` | `(claim_data, polygon_id) -> bool` | Verify full claim |
| `prove_polygon_containment` | `(token_id, polygon_id) -> bool` | Check polygon binding |
| `prove_claim` | `(wallet, period_start, period_end) -> (root, proof, leaf_index)` | Merkle proof |
| `set_verifier` | `(contract_id, authorized)` | Auth verifier (admin) |
| `get_retired_polygons` | `(claimer) -> Vec<BytesN<32>>` | List claimer's polygons |

### Marketplace Contract

| Function | Signature | Description |
|----------|-----------|-------------|
| `__constructor` | `(admin, rec_token_addr, usdc_addr, fee_vault, default_fee_rate_bps)` | Initialize |
| `place_order` | `(side, price, qty, restrictions, biome_filter, vintage_filter) -> u64` | Place order |
| `cancel_order` | `(order_id)` | Cancel order |
| `get_order` | `(order_id) -> Order` | Query order |
| `match_orders` | `(buy_id, sell_id) -> (u64, i128, i128)` | Match orders |
| `auto_match` | `() -> u32` | Auto-match crossed orders |
| `get_best_bid` | `() -> Option<Order>` | Best buy |
| `get_best_ask` | `() -> Option<Order>` | Best sell |
| `open_cfd` | `(counterparty, strike, qty, settlement_date, collateral, oracle_feed_id, biome, maintenance_bps) -> u64` | Open CfD |
| `accept_cfd` | `(position_id, collateral)` | Accept CfD |
| `settle_cfd` | `(position_id, spot_price) -> i128` | Settle CfD |
| `liquidate_cfd` | `(position_id)` | Liquidate position |
| `set_fee_rate` | `(bps)` | Set fee (admin, capped) |

---

## 🔒 Security & Risk

### Smart Contract Risks

| Risk | Mitigation |
|------|-----------|
| **Oracle collusion** | N-of-M threshold with slashing; economic disincentive > potential fraud gain |
| **Survey data fraud** | Cross-validation between independent analysis methods; dispute window with expert review |
| **Governance capture** | Community veto cannot be overridden; weighted voting prevents single-actor control |
| **Reentrancy** | Soroban's contract model prevents reentrancy by design (no reentrant calls during execution) |
| **Integer overflow** | Safe math (Rust checked arithmetic); `i128` for financial calculations |
| **Admin key compromise** | Transferable admin; multi-sig planned for production |
| **IPFS data loss** | Multiple pinning services (Pinata, Filecoin, web3.storage); CID integrity verified on-chain |
| **Polygon geometry fraud** | Bounding box on-chain → satellite imagery cross-reference by oracles |
| **Double minting** | `polygon_id + survey_timestamp` uniqueness constraint |

### Audit Status

- **Smart contracts**: Internal audit completed; external audit TBD (see roadmap)
- **Oracle network**: Threshold BLS signature scheme reviewed; formal verification planned
- **Frontend**: No sensitive key management (transactions signed by wallet extension)

---

## 🗺 Roadmap

### Phase 1 — Foundation (Current)
- [x] Concept & architecture
- [ ] Smart contract scaffolding (workspace, types, errors)
- [ ] BDC Token contract (mint, burn, transfer, metadata)
- [ ] MRV Oracle contract (registration, submit, verify)
- [ ] Approval Governance contract (proposal, voting, veto)
- [ ] Retirement Registry contract (retire, polygon, verify)
- [ ] Marketplace contract (order book, matching, CfD)
- [ ] Integration tests (full lifecycle)

### Phase 2 — Pilot (Q3 2026)
- [ ] Testnet deployment
- [ ] Initial methodology: Tropical Forest (BDCX-TF-v1.0)
- [ ] 3 pilot projects (Amazon, Sulawesi, Kenya)
- [ ] Frontend dashboard (geo-spatial, marketplace, portfolio)
- [ ] External audit (smart contracts)

### Phase 3 — Launch (Q4 2026)
- [ ] Mainnet deployment
- [ ] Methodology v1.0 suite (Forest, Mangrove, Grassland)
- [ ] Oracle node operator onboarding
- [ ] Corporate buyer onboarding toolkit
- [ ] TNFD/SBTN alignment guide publication

### Phase 4 — Growth (2027)
- [ ] DAO governance transition
- [ ] Cross-chain bridges (Celo, Polygon, Regen)
- [ ] Advanced CfD liquidation engine
- [ ] ZK-proof integration for privacy-preserving claims
- [ ] Mobile app for community stakeholders

### Phase 5 — Scale (2028+)
- [ ] Automated MRV (real-time sensor streams)
- [ ] AI-powered fraud detection (oracle behavior scoring)
- [ ] Biodiversity derivatives market
- [ ] Insurance products for conservation outcomes
- [ ] Global methodology registry

---

## ❓ FAQ

**Q: How is a biodiversity credit different from a carbon credit?**
A: Carbon credits represent 1 tonne of CO₂ avoided/removed. Biodiversity credits represent verifiable improvements in ecosystem health — species richness, habitat connectivity, ecosystem function. A single biodiversity credit bundles multiple ecological metrics (BSI) rather than a single atmospheric gas.

**Q: Who determines if a credit is valid?**
A: No single actor. The MRV Oracle network verifies raw ecological data (eDNA, camera traps, satellite). Then a multi-stakeholder panel (ecologists, community representatives, auditors) votes on credit issuance. Community members have veto power.

**Q: How do you prevent double-counting?**
A: Each BDC token has a unique `token_id` and references a specific `polygon_id`. When retired, the token is burned (removed from circulation) and the retirement is recorded immutably on-chain with a public receipt. Any verifier can check whether a specific token has been retired.

**Q: Can local communities really veto a project?**
A: Yes. The community stakeholder role has on-chain veto power. If the community representative votes "no" on a proposal, it is automatically rejected — no other stakeholder can override this. This encodes Free, Prior, and Informed Consent (FPIC) directly into the smart contract.

**Q: What ecological data is stored on-chain?**
A: Minimal data. Raw survey data (eDNA sequences, camera trap images, audio recordings) is stored on IPFS and referenced by its content hash (CID). On-chain, we store only the CID, bounding box coordinates, and aggregated metrics (BSI values, species counts). This preserves data privacy while maintaining verifiability.

**Q: How does polygon anchoring work?**
A: Full GPS polygon geometry is stored on IPFS. On-chain, we store a bounding box (min/max lat/lon) plus the IPFS CID. When credits are retired, the retirement receipt includes the polygon ID and bounding box. Verifiers can cross-reference the on-chain bounding box with the IPFS polygon geometry and satellite imagery.

**Q: What's the minimum polygon size?**
A: 50 ha minimum (configurable per methodology). Maximum polygon size is unlimited, but practical MRV constraints suggest 10,000 ha as a practical upper limit for a single polygon.

**Q: How are oracle nodes incentivized?**
A: Oracles earn $5 per validated survey attestation (distributed from protocol fees). They also stake tokens that can be slashed for fraudulent attestations. Oracle reputation scores are tracked on-chain, affecting future earning potential.

---

## 🤝 Contributing

We welcome contributions from ecologists, blockchain developers, conservation finance experts, and community representatives!

Before contributing, please read our:
- [Contributing Guidelines](./CONTRIBUTING.md)
- [Code of Conduct](./CODE_OF_CONDUCT.md)
- [Security Policy](./SECURITY.md)

### Reporting Issues

- **Bug reports**: Use our [Bug Report template](.github/ISSUE_TEMPLATE/bug_report.md).
- **Feature requests**: Use our [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.md).
- **Security vulnerabilities**: Please see our [Security Policy](./SECURITY.md).

### Quick Start for Developers

```bash
# Build and test
cargo build --release
cargo test --all-features
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

---

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**🌿 Biodiversity Credit Exchange**

*Tokenize conservation. Trade biodiversity. Retire on polygons.*

[Getting Started](#-getting-started) · [Documentation](docs/) · [Contributing](#-contributing) · [Roadmap](#-roadmap)

</div>
