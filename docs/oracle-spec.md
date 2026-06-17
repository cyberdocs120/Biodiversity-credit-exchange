# Oracle Network Specification

## Overview

The MRV (Measurement, Reporting, and Verification) Oracle Network is a decentralized network of biodiversity monitoring nodes that submit validated survey data to the BDCX protocol. Surveys are verified through N-of-M threshold signatures and cross-validation between oracle types.

## Oracle Network Topology

```
                         ┌──────────────────────────┐
                         │      BDCX Contracts       │
                         │   (mrv-oracle + gov)      │
                         └──────────────────────────┘
                                    ▲
                                    │ Survey submission
                                    │ (N-of-M sigs)
           ┌────────────────────────┼────────────────────────┐
           │                        │                        │
           ▼                        ▼                        ▼
   ┌──────────────┐       ┌──────────────┐       ┌──────────────┐
   │   eDNA Lab   │       │ Camera Trap  │       │  Satellite   │
   │   Oracle     │       │ AI Oracle    │       │  Imagery     │
   └──────────────┘       └──────────────┘       └──────────────┘
           ▲                        ▲                        ▲
           │                        │                        │
           ▼                        ▼                        ▼
   ┌──────────────┐       ┌──────────────┐       ┌──────────────┐
   │ Field Survey │       │  Acoustic    │       │  Future Types│
   │   Oracle     │       │  Sensor      │       │  (Extensible)│
   └──────────────┘       └──────────────┘       └──────────────┘
```

## Oracle Types

| Code | Type             | Description                           | Typical Frequency |
|------|------------------|---------------------------------------|-------------------|
| 0    | EdnaLab          | Environmental DNA lab analysis        | Quarterly         |
| 1    | CameraTrapAi     | AI-powered camera trap image analysis | Monthly           |
| 2    | SatelliteImagery | NDVI / remote sensing analysis        | Monthly           |
| 3    | FieldSurvey      | Manual ecologist field survey         | Semi-annual       |
| 4    | AcousticSensor   | Audio-based biodiversity monitoring   | Monthly           |

### OracleNode Struct

```rust
pub struct OracleNode {
    pub pubkey: BytesN<32>,       // Ed25519 public key
    pub uri: Bytes,               // Off-chain metadata endpoint
    pub oracle_type: OracleType,  // Type classification
    pub active: bool,             // Whether oracle is currently active
    pub registered_at: u64,       // Registration timestamp
    pub total_surveys: u64,       // Lifetime survey count
    pub accuracy_score: u32,      // 0–100 accuracy rating
}
```

## Threshold Voting (N-of-M)

Each polygon has a configurable threshold `(N, M)` where:
- **N** = minimum number of valid oracle signatures required
- **M** = total number of registered oracles (or relevant subset)

Default: `(2, 3)` — requires at least 2 valid signatures from 3 registered oracles.

### Configuration

- Set by admin via `set_threshold(n, d)` where threshold = n/d
- Stored as separate numerator/denominator to avoid floating-point
- On submission: `signatures.len() >= n` must be satisfied
- At least `n` distinct, active oracles must have signed

## Signature Verification Flow

```
1. Oracle generates survey data
2. Oracle signs: SHA256(polygon_id || ipfs_cid || timestamp)
3. Oracle submits to network with { polygon_id, ipfs_cid, timestamp, signatures[] }
4. Contract verifies each signature:
   a. Look up pubkey in oracle registry
   b. Check oracle is active
   c. Recover message hash from signed payload
   d. Verify Ed25519 signature against pubkey
5. Count valid signatures; reject if < N
6. Create survey record and emit event
```

### Signature Format

```rust
// Each signature is a tuple (pubkey: BytesN<32>, signature: BytesN<64>)
pub type Signature = (BytesN<32>, BytesN<64>);
```

- `pubkey` — Ed25519 public key of the signing oracle
- `signature` — 64-byte Ed25519 signature over the survey hash

### Survey Hash Computation

```rust
let mut data = Bytes::new(&env);
data.append(&Bytes::from(&env, polygon_id.as_ref()));
data.append(&ipfs_cid);
data.append(&timestamp_be_bytes);
let survey_hash: BytesN<32> = env.crypto().sha256(data);
```

## IPFS Content Addressing

All survey data is stored off-chain on IPFS. The CID submitted on-chain must resolve to valid survey data.

### IPFS CID Verification

When a survey is submitted with an IPFS CID, the contract stores the CID but does not verify its content on-chain (content verification is an off-chain process by oracle operators and validators).

### Data Structure (IPFS)

Each IPFS-pinned survey contains:

```json
{
  "survey_version": "1.0",
  "polygon_id": "0x...",
  "survey_timestamp": 1718000000,
  "oracle_type": "CameraTrapAi",
  "oracle_pubkey": "0x...",
  "analyses": [
    {
      "analysis_type": "species_richness",
      "value": 0.72,
      "confidence": 0.95,
      "raw_data_cid": "bafy..."
    },
    {
      "analysis_type": "shannon_diversity",
      "value": 0.58,
      "confidence": 0.90,
      "raw_data_cid": "bafy..."
    }
  ],
  "bsi_components": {
    "sr": 0.72,
    "sdi": 0.58,
    "cc": 0.65,
    "iucn": 0.40,
    "fq": 0.55
  },
  "bsi_total": 0.614
}
```

## Cross-Validation Tolerances

When multiple oracles survey the same polygon within a validation window, tolerances apply:

| Oracle Pair                    | BSI Tolerance | Cooldown Period |
|--------------------------------|---------------|-----------------|
| eDNA Lab ↔ Field Survey        | ±0.05         | 7 days          |
| Satellite Imagery ↔ Camera Trap AI | ±0.08     | 3 days          |
| Camera Trap AI ↔ Field Survey  | ±0.06         | 14 days         |
| Acoustic Sensor ↔ Field Survey | ±0.07         | 7 days          |
| Satellite Imagery ↔ Field Survey | ±0.10       | 14 days         |

## Dispute Resolution Flow

```
                 ┌─────────────────────┐
                 │  Survey Submitted    │
                 │  (meets threshold)   │
                 └──────────┬──────────┘
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
   ┌──────────────────┐       ┌──────────────────┐
   │    No Dispute     │      │   Dispute Filed   │
   │   Normal flow     │      │   (by any oracle) │
   └──────────────────┘       └────────┬─────────┘
                                       │
                                       ▼
                            ┌──────────────────────┐
                            │   Admin Review        │
                            │   (resolve_dispute)   │
                            └──────────┬───────────┘
                                       │
                          ┌────────────┴────────────┐
                          │                         │
                          ▼                         ▼
               ┌──────────────────┐       ┌──────────────────┐
               │   Valid Survey    │       │   Fraud Detected │
               │   (resolved=true) │       │   (slash oracles)│
               └──────────────────┘       └──────────────────┘
```

### Dispute Flow

1. Any oracle can call `dispute(survey_hash)` on a survey
2. The survey's `disputed` flag is set to `true`
3. The associated proposal in approval-gov is paused
4. Admin reviews the dispute off-chain
5. Admin calls `resolve_dispute(survey_hash, outcome, slashed_oracles[])`
6. If outcome is fraud, specified oracles are slashed

### Slashing Conditions

| Violation                    | Slash Amount         | Description                              |
|------------------------------|----------------------|------------------------------------------|
| Fraudulent survey data       | 100% of bond         | Intentional false data submission        |
| Repeated accuracy < 30%      | 50% of bond          | Poor quality submissions                 |
| Signature key compromise     | Full revocation      | Key rotation required to re-register     |
| Collusion with other oracles | 100% of bond + ban   | Coordinated fraud                        |

## SurveyRecord Structure

```rust
pub struct SurveyRecord {
    pub survey_hash: BytesN<32>,       // Unique hash identifier
    pub polygon_id: BytesN<32>,        // Associated polygon
    pub ipfs_cid: Bytes,               // IPFS content identifier
    pub survey_timestamp: u64,         // When survey was conducted
    pub oracle_count: u32,             // Number of valid signatures
    pub threshold_met: bool,           // Whether N-of-M was satisfied
    pub disputed: bool,                // Dispute flag
    pub resolved: bool,                // Resolution flag
    pub token_ids: Vec<u64>,           // Minted BDC token IDs
    pub analyses_hashes: Vec<BytesN<32>>,  // Per-oracle analysis hashes
}
```

## SurveyData (Submission Payload)

```rust
pub struct SurveyData {
    pub polygon_id: BytesN<32>,
    pub ipfs_cid: Bytes,
    pub survey_timestamp: u64,
    pub signatures: Vec<(BytesN<32>, BytesN<64>)>,
    pub analyses_hashes: Vec<BytesN<32>>,
    pub baseline_bsi: u32,
    pub current_bsi: u32,
    pub area_contribution: u64,
    pub biome: u32,
    pub vintage_year: u16,
    pub vintage_qtr: u8,
    pub methodology_id: BytesN<8>,
    pub beneficiary: Address,
}
```

## Oracle Registration Flow

```
1. Admin generates oracle keypair (Ed25519)
2. Admin registers oracle via register_oracle(pubkey, uri, oracle_type)
3. Oracle registered with active=true, accuracy_score=100
4. Admin sets threshold via set_threshold(n, d)
5. Oracles can now submit surveys for registered polygons
6. Admin can revoke oracles at any time via revoke_oracle(pubkey)
7. Revoked oracles cannot submit new surveys
```

## Emergency Controls

Admin can pause all survey submissions via `pause()`:

- `pause()` — Sets contract paused flag, all submissions rejected
- `resume()` — Clears paused flag, normal operations resume
- Only effective for survey submission; read operations unaffected
