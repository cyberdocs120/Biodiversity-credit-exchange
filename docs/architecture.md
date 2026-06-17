# Architecture

## Layer Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                      Frontend (React + MapLibre)                 │
│  Dashboard  │  Marketplace  │  Portfolio  │  Polygon Map Viewer  │
└──────────────────────────┬───────────────────────────────────────┘
                           │ Soroban RPC
┌──────────────────────────▼───────────────────────────────────────┐
│                     Soroban Smart Contracts                      │
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌────────────┐  ┌──────────────┐  │
│  │ bdc-token│◄─│   mrv-   │─►│ approval-  │  │  retirement  │  │
│  │ (SEP-41) │  │  oracle  │  │    gov     │  │  (registry)  │  │
│  └──┬───▲───┘  └──────────┘  └──────┬──────┘  └──────▲───────┘  │
│     │   │                           │                 │          │
│     │   └───────────────────────────┘                 │          │
│     ▼                                                   │          │
│  ┌──────────────────────────────────────────────────────┘          │
│  │                     marketplace                                 │
│  │        (order book + matching engine + CfD)                     │
│  └──────────────────────────────────────────────────────────────────┘
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
                           │ Stellar Consensus
┌──────────────────────────▼───────────────────────────────────────┐
│                        Stellar Network                           │
│              (Soroban Host Functions + Stellar Core)              │
└──────────────────────────────────────────────────────────────────┘
```

## Crate Dependency Graph

```
bdc-token      (standalone token — no internal deps)
    ▲
    │
mrv-oracle     (calls bdc-token.mint and approval-gov.propose)
    ▲
    │
approval-gov   (calls bdc-token.mint, reads mrv-oracle data)
    ▲
    │
retirement     (calls bdc-token.burn)
    ▲
    │
marketplace    (calls bdc-token.transfer, yUSDC.transfer)
```

- `bdc-token` has zero contract dependencies.
- `mrv-oracle` depends on `bdc-token` (for minting) and `approval-gov` (for proposal creation).
- `approval-gov` depends on `bdc-token` (for minting credits on approval) and `mrv-oracle` (for survey/polygon data).
- `retirement` depends on `bdc-token` (for burning tokens on retirement).
- `marketplace` depends on `bdc-token` (for transferring BDC tokens) and an external yUSDC token.

## Cross-Contract Call Patterns

| Caller                  | Callee                | Function     | Trigger                      |
|-------------------------|-----------------------|--------------|------------------------------|
| `mrv-oracle`            | `bdc-token`           | `mint`       | Survey approved (via gov)    |
| `mrv-oracle`            | `approval-gov`        | `propose`    | Submit survey with threshold |
| `approval-gov`          | `bdc-token`           | `mint`       | Proposal approved            |
| `retirement`            | `bdc-token`           | `burn`       | Retire batch                 |
| `marketplace`           | `bdc-token`           | `transfer`   | Match orders (seller→buyer)  |
| `marketplace`           | `yUSDC` (external)    | `xfer`       | Payment settlement           |

## Storage Architecture

All contracts follow a consistent storage convention using Soroban's `Env::storage()` API:

### Instance Storage (singleton keys)
Used for admin addresses, configuration values, and counters. Keys are 4-byte `Symbol` values:

| Symbol  | Type      | Used By                        |
|---------|-----------|--------------------------------|
| `Admin` | `Address` | All contracts                  |
| `RecT`  | `Address` | mrv-oracle, approval-gov, retirement, marketplace |
| `ThN`   | `u32`     | mrv-oracle (threshold num)     |
| `ThD`   | `u32`     | mrv-oracle (threshold den)     |
| `OrC`   | `u32`     | mrv-oracle (oracle count)      |
| `Pause` | `bool`    | mrv-oracle                     |
| `MinW`  | `u32`     | approval-gov (min weight)      |
| `VPer`  | `u64`     | approval-gov (voting period)   |
| `PrCN`  | `u64`     | approval-gov (proposal cnt)    |
| `StCN`  | `u64`     | approval-gov (stakeholder cnt) |
| `FeeRt` | `u16`     | marketplace (fee rate)         |
| `OrCN`  | `u64`     | marketplace (order counter)    |
| `TIDC`  | `u64`     | bdc-token (token counter)      |
| `TSup`  | `u64`     | bdc-token (total supply)       |
| `AMnt`  | `Address` | bdc-token (authorized minter)  |
| `ABrn`  | `Address` | bdc-token (authorized burner)  |
| `FVal`  | `Address` | marketplace (fee vault)        |
| `USDC`  | `Address` | marketplace (yUSDC address)    |
| `Gov`   | `Address` | mrv-oracle (approval-gov)      |
| `MrvO`  | `Address` | approval-gov (mrv-oracle)      |
| `RcCN`  | `u64`     | retirement (receipt counter)   |

### Persistent Storage (key-value index)
Used for records indexed by ID/hash. All use a 1-byte prefix + key bytes:

| Prefix | Value Type           | Contract         | Description                 |
|--------|----------------------|------------------|-----------------------------|
| `0x01` | `BdcTokenValue`      | bdc-token        | Token metadata by token_id  |
| `0x02` | `u64` (count)        | bdc-token        | Token count by owner        |
| `0x03` | `u64` (count)        | bdc-token        | Token count by polygon_id   |
| `0x10` | `OracleNode`         | mrv-oracle       | Oracle by pubkey            |
| `0x20` | `HabitatPolygon`     | mrv-oracle       | Polygon by polygon_id       |
| `0x30` | `SurveyRecord`       | mrv-oracle       | Survey by survey_hash       |
| `0x10` | `Stakeholder`        | approval-gov     | Stakeholder by address      |
| `0x20` | `Proposal`           | approval-gov     | Proposal by proposal_id     |
| `0x10` | `RetirementReceipt`  | retirement       | Receipt by receipt_id       |
| `0x20` | `bool`               | retirement       | Retired flag by token_id    |
| `0x30` | `Vec<BytesN<32>>`    | retirement       | Receipt IDs by polygon+retirer |

### Temporary Storage
Not used in current contracts. All state is either `Instance` or `Persistent`.

## Deployment Topology (Initialization Order)

Contracts must be deployed and initialized in a specific order due to cross-contract references:

```
Step 1: Deploy & init bdc-token
Step 2: Deploy & init mrv-oracle
Step 3: Set bdc-token address on mrv-oracle
Step 4: Deploy & init approval-gov
Step 5: Set bdc-token and mrv-oracle addresses on approval-gov
Step 6: Set approval-gov address on mrv-oracle
Step 7: Authorize approval-gov as minter on bdc-token
Step 8: Deploy & init retirement
Step 9: Set bdc-token address on retirement
Step 10: Deploy & init marketplace
Step 11: Set bdc-token, yUSDC, and fee vault on marketplace
```

## Event Architecture

All contracts emit typed events using Soroban's `env.events().publish()`:

| Contract      | Event Topics                              | Payload                                 |
|---------------|-------------------------------------------|-----------------------------------------|
| bdc-token     | `("bdct","mint")`, `("bdct","tran")`, `("bdct","burn")` | token_id, from, to, polygon_id |
| mrv-oracle    | `("mrvo","reg")`, `("mrvo","rev")`, `("mrvo","poly")` | pubkey/polygon_id, timestamp           |
|               | `("mrvo","subm")`, `("mrvo","disp")`, `("mrvo","resd")` | survey_hash, outcome                    |
| approval-gov  | `("gov","prop")`, `("gov","vote")`, `("gov","veto")` | proposal_id, voter, state               |
|               | `("gov","appr")`, `("gov","rejc")`       | proposal_id, token_id range             |
| retirement    | `("retr","done")`                         | receipt_id, polygon_id, count, total    |
| marketplace   | `("mkt","plac")`, `("mkt","canc")`, `("mkt","matc")` | order_id(s), side, price, qty           |

## Key Design Decisions

1. **#![no_std]** — All contracts are `#![no_std]` for WASM compatibility.
2. **require_auth()** — All privileged operations require `env.invoker().require_auth()` or `admin.require_auth()`.
3. **Typed errors** — Every contract exports a `#[contracterror]` enum for structured revert reasons.
4. **Pagination** — List queries (`tokens_by_owner`, `tokens_by_polygon`) support `start`/`limit` pagination.
5. **Fixed-point arithmetic** — All coordinates use `i64` with 1,000,000 multiplier for sub-meter precision.
6. **Fee model** — Marketplace charges a flat fee rate (default 25 bps, max 100 bps) on matched trades, sent to a configurable fee vault.
