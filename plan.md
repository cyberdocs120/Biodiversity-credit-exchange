# 🚀 Biodiversity Credit Exchange — 16-Day Development Sprint (→ 55% Completion)

> **Goal**: Build a robust, contributor-ready foundation covering all five Soroban smart contracts, integration tests, deployment scripts, and essential documentation.  
> **Target**: 55% of the full project vision (as defined in `README.md`).  
> **Remaining 45%**: Advanced CfD liquidation engine, full frontend with geo-map, fuzz testing, governance/DAO, ZK-proof integration, cross-chain bridges, and production hardening.

---

## 📐 Scope: What "55%" Looks Like

| Component | % of Total | Status after Sprint |
|-----------|-----------|---------------------|
| Cargo workspace, CI, tooling | 3% | ✅ 100% |
| `bdc-token` contract (SEP-41) | 10% | ✅ 100% — mint, burn, transfer, metadata, polygon binding, tests |
| `mrv-oracle` contract | 12% | ✅ 100% — registration, N-of-M threshold, IPFS verification, submit/dispute, tests |
| `approval-gov` contract | 15% | ✅ 100% — stakeholder registry, weighted voting, community veto, proposal lifecycle, tests |
| `retirement` contract | 12% | ✅ 100% — retire batch, polygon anchoring, merkle proof, verify claim, geometry containment, tests |
| `marketplace` + CfD engine | 18% | 🔶 ~50% — spot order book complete; CfD open/close/settle done; liquidation engine partial |
| Integration tests | 8% | 🔶 ~50% — core cross-contract flows covered |
| Scripts (deploy, setup) | 4% | ✅ 100% |
| Documentation | 6% | 🔶 ~40% — architecture + methodology + polygon spec + oracle spec |
| Frontend (React + geo-map) | 8% | 🔶 ~10% — scaffold only (App shell, 1–2 pages) |
| CI/CD | 2% | ✅ 100% |

**Weighted total: ≈55%**

---

## 📅 Day-by-Day Plan

### Week 1 — Smart Contract Core

---

#### Day 1 — Workspace Scaffolding

**Prompt for agent**: Create the Cargo workspace, all 6 crate directories, `.gitignore`, Rust toolchain, and CI pipeline.

**Files to create**:

```
.gitignore
rust-toolchain.toml
Cargo.toml                              (root workspace)
contracts/bdc-token/Cargo.toml
contracts/mrv-oracle/Cargo.toml
contracts/approval-gov/Cargo.toml
contracts/retirement/Cargo.toml
contracts/marketplace/Cargo.toml
tests/integration/Cargo.toml
contracts/bdc-token/src/lib.rs          (hello-world placeholder)
contracts/mrv-oracle/src/lib.rs         (hello-world placeholder)
contracts/approval-gov/src/lib.rs       (hello-world placeholder)
contracts/retirement/src/lib.rs         (hello-world placeholder)
contracts/marketplace/src/lib.rs        (hello-world placeholder)
tests/integration/src/lib.rs            (empty)
.github/workflows/ci.yml
.github/dependabot.yml
```

**`.gitignore`**:

```
target/
Cargo.lock
.wasm/
*.wasm
frontend/node_modules/
frontend/.next/
frontend/build/
.env
.env.local
.vscode/
.idea/
*.swp
*.swo
.DS_Store
Thumbs.db
```

**`rust-toolchain.toml`**:

```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

**Root `Cargo.toml`**:

```toml
[workspace]
members = [
    "contracts/bdc-token",
    "contracts/mrv-oracle",
    "contracts/approval-gov",
    "contracts/retirement",
    "contracts/marketplace",
    "tests/integration",
]

[workspace.package]
edition = "2021"
rust-version = "1.81"
license = "MIT"

[workspace.dependencies]
soroban-sdk = "22.0.0"
```

**Per-contract `Cargo.toml`** (same pattern for all 5):

Each crate has `crate-type = ["cdylib"]` and depends on `soroban-sdk.workspace = true`. Only `name` changes.

**bdc-token**:
```toml
[package]
name = "bdc-token"
version = "0.1.0"
edition.workspace = true

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk.workspace = true
```

**mrv-oracle**: same but `name = "mrv-oracle"`  
**approval-gov**: same but `name = "approval-gov"`  
**retirement**: same but `name = "retirement"`  
**marketplace**: same but `name = "marketplace"`  

**`tests/integration/Cargo.toml`**:

```toml
[package]
name = "integration-tests"
version = "0.1.0"
edition.workspace = true

[dependencies]
soroban-sdk.workspace = true
bdc-token = { path = "../../contracts/bdc-token" }
mrv-oracle = { path = "../../contracts/mrv-oracle" }
approval-gov = { path = "../../contracts/approval-gov" }
retirement = { path = "../../contracts/retirement" }
marketplace = { path = "../../contracts/marketplace" }
```

**Placeholder `src/lib.rs`** for each contract crate:

Each gets:
```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn hello(env: Env) -> soroban_sdk::String {
        soroban_sdk::String::from_str(&env, "hello")
    }
}
```

Replace `MyContract` with `BdcTokenContract`, `MrvOracleContract`, `ApprovalGovContract`, `RetirementContract`, `MarketplaceContract` respectively.

**`tests/integration/src/lib.rs`**:

```rust
#![no_std]
```

**`.github/workflows/ci.yml`**:

```yaml
name: CI
on: [push, pull_request]
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo check --all-targets
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo test --all-features
  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo clippy --all-targets -- -D warnings
  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - run: cargo fmt --check
```

**`.github/dependabot.yml`**:

```yaml
version: 2
updates:
  - package-ecosystem: cargo
    directory: "/"
    schedule:
      interval: weekly
  - package-ecosystem: github-actions
    directory: "/"
    schedule:
      interval: monthly
```

**Verification**:

```bash
cargo check --all-targets
# Must pass with zero errors.
```

---

#### Day 2 — 

---

#### Day 3 — 
```

---

#### Day 4 — BDC Token: Metadata & Query Edge Cases

**Prompt for agent**: Add `set_metadata_uri`, `tokens_by_owner`, `tokens_by_polygon`, and edge-case tests.

**Files to modify**: `contracts/bdc-token/src/lib.rs`, `contracts/bdc-token/src/test.rs`  
**Prerequisite**: Day 3 done.

**New functions in `lib.rs`**:

**`set_metadata_uri(env, token_id, new_uri)`**:
- Admin-only (`admin.require_auth()`)
- Check token exists, update `metadata_uri`, write back

**`tokens_by_owner(env, owner, start, limit) -> Vec<u64>`**:
- Scan token IDs from 1 to `read_token_id_counter`
- Collect tokens where `owner == given_owner && state == Active`
- Respect pagination (start offset, limit count)
- Return `Vec<u64>`

**`tokens_by_polygon(env, polygon_id, start, limit) -> Vec<u64>`**:
- Same pattern but filtered by `polygon_id`
- Pagination support

**Additional tests**:

| Test | What it checks |
|------|---------------|
| `test_set_metadata_uri` | URI changes, admin can update |
| `test_tokens_by_owner_pagination` | Returns correct slices |
| `test_tokens_by_polygon` | Filters by polygon_id correctly |
| `test_transfer_retired_token_rejected` | `#[should_panic(expected = "BdcAlreadyRetired")]` |
| `test_mint_emits_event` | Verify event data |

**Verification**:

```bash
cargo test -p bdc-token
# 12+ tests pass
```

---

#### Day 5 — MRV Oracle: Oracle Management & Polygon Registration

**Prompt for agent**: Implement oracle registration/revocation, threshold, polygon registration, pause/resume.

**Files to create**: `contracts/mrv-oracle/src/{lib,storage,types,errors,test}.rs`  
**Prerequisite**: Day 3 done (bdc-token functional).

**`contracts/mrv-oracle/src/types.rs`**:

```rust
#[derive(Clone, Debug)]
#[contracttype]
pub enum OracleType {
    EdnaLab = 0,
    CameraTrapAi = 1,
    SatelliteImagery = 2,
    FieldSurvey = 3,
    AcousticSensor = 4,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct OracleNode {
    pub pubkey: BytesN<32>,
    pub uri: Bytes,
    pub oracle_type: OracleType,
    pub active: bool,
    pub registered_at: u64,
    pub total_surveys: u64,
    pub accuracy_score: u32,  // 0-100
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct HabitatPolygon {
    pub polygon_id: BytesN<32>,
    pub geometry_ipfs_cid: Bytes,
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

#[derive(Clone, Debug)]
#[contracttype]
pub struct SurveyRecord {
    pub survey_hash: BytesN<32>,
    pub polygon_id: BytesN<32>,
    pub ipfs_cid: Bytes,
    pub survey_timestamp: u64,
    pub oracle_count: u32,
    pub threshold_met: bool,
    pub disputed: bool,
    pub resolved: bool,
    pub token_ids: Vec<u64>,
    pub analyses_hashes: Vec<BytesN<32>>,
}
```

**`contracts/mrv-oracle/src/errors.rs`**:

```rust
#[contracterror]
pub enum MrvOracleError {
    Unauthorized = 1,
    OracleAlreadyRegistered = 2,
    OracleNotFound = 3,
    ThresholdNotMet = 4,
    InvalidSignature = 5,
    InvalidSurveyData = 6,
    PolygonNotFound = 7,
    PolygonInactive = 8,
    SurveyNotFound = 9,
    SurveyAlreadyResolved = 10,
    DuplicateSurvey = 11,
    ContractPaused = 12,
}
```

**`contracts/mrv-oracle/src/storage.rs`**:

Keys (use `Symbol::short` for singleton, `Bytes` prefix for indexed):

| Key | Type | Persistence |
|-----|------|-------------|
| `Symbol::short("Admin")` | `Address` | Instance |
| `Symbol::short("Pause")` | `bool` | Instance |
| `Symbol::short("ThN")` / `"ThD"` | `u32` | Instance |
| `Symbol::short("OrC")` | `u32` (oracle count) | Instance |
| `Symbol::short("RecT")` | `Address` (bdc-token addr) | Instance |
| `0x10 + pubkey` bytes | `OracleNode` | Persistent |
| `0x20 + polygon_id` bytes | `HabitatPolygon` | Persistent |
| `0x30 + survey_hash` bytes | `SurveyRecord` | Persistent |

Storage helpers for each of the above (write/read/has pattern).

**`contracts/mrv-oracle/src/lib.rs`**:

Functions to implement:

| Function | Auth | Logic |
|----------|------|-------|
| `__constructor(admin)` | admin | Store admin, paused=false, threshold=1/1, oracle_count=0 |
| `admin()` | — | Read admin |
| `transfer_admin(new)` | admin+new | Both require_auth |
| `set_bdc_token(addr)` | admin | Store bdc-token address |
| `bdc_token()` | — | Read bdc-token address |
| `register_oracle(pubkey, uri, oracle_type)` | admin | Check not duplicate, create `OracleNode`, increment count, emit `("mrvo","reg")` |
| `revoke_oracle(pubkey)` | admin | Check exists, set `active = false`, emit `("mrvo","rev")` |
| `oracle_count()` | — | Read count |
| `get_oracle(pubkey)` | — | Check exists, return |
| `set_threshold(n, d)` | admin | Validate n>0, d>0, n<=d. Store |
| `threshold()` | — | Return (n, d) |
| `register_polygon(polygon_id, geometry_cid, bbox, area_ha, biome, country, project_id)` | admin | Create polygon, store, emit `("mrvo","poly")` |
| `close_polygon(polygon_id)` | admin | Set active=false, emit |
| `get_polygon(polygon_id)` | — | Check exists, return |
| `pause()` | admin | Set paused=true, emit |
| `resume()` | admin | Set paused=false, emit |
| `paused()` | — | Read paused flag |

Leave `submit_survey`, `dispute`, `resolve_dispute` as stubs (panic "not implemented").

**`contracts/mrv-oracle/src/test.rs`**:

Tests:
- `test_register_oracle` — count=1, node.active=true
- `test_revoke_oracle` — node.active=false
- `test_set_threshold` — read back 3,5
- `test_pause_resume` — paused() toggles
- `test_register_polygon` — read back correct fields
- `test_close_polygon` — active=false
- `test_duplicate_oracle_rejected` — `#[should_panic(expected = "OracleAlreadyRegistered")]`
- `test_get_nonexistent_oracle` — `#[should_panic(expected = "OracleNotFound")]`

**Verification**:

```bash
cargo test -p mrv-oracle
# 8 tests pass
```

---

#### Day 6 — MRV Oracle: Submit Survey & Dispute Resolution

**Prompt for agent**: Implement `submit_survey` with IPFS content verification, N-of-M signature validation, and cross-contract call to approval governance. Add dispute/resolve.

**Files to modify**: `contracts/mrv-oracle/src/lib.rs` (replace stubs), test.rs (add tests)  
**Prerequisite**: Day 5 done. Day 3 done (bdc-token mint works).

**`submit_survey` specification**:

```rust
pub fn submit_survey(
    env: Env,
    polygon_id: BytesN<32>,
    ipfs_cid: Bytes,
    survey_timestamp: u64,
    signatures: Vec<(BytesN<32>, BytesN<64>)>,  // (pubkey, signature) pairs
    analyses_hashes: Vec<BytesN<32>>,            // per-oracle analysis hash
    baseline_bsi: u32,
    current_bsi: u32,
    area_contribution: u64,
    biome: u32,
    vintage_year: u16,
    vintage_qtr: u8,
) -> u64
```

Logic:
1. Check `!read_paused()` else panic `ContractPaused`
2. Check polygon exists via `has_polygon`, else panic `PolygonNotFound`
3. Check polygon is active, else panic `PolygonInactive`
4. Read threshold `(n, d)`. Check `signatures.len() >= n` else panic `ThresholdNotMet`
5. Validate each signature's pubkey: `has_oracle` and `oracle.active` else panic `InvalidSignature`
6. Compute survey hash via `env.crypto().sha256(Bytes)` from `polygon_id + ipfs_cid + survey_timestamp_be_bytes`
7. Check `!has_survey` else panic `DuplicateSurvey`
8. Create `SurveyRecord`, store it
9. Emit `SurveySubmitted` event
10. Cross-contract call to approval-gov's `propose()` to begin multi-stakeholder approval
11. Return survey_hash

Cross-contract call pattern:
```rust
let gov_id = read_approval_gov(&env);
let proposal_id: u64 = env.invoke_contract(
    &gov_id,
    &Symbol::short("prop"),
    (polygon_id.clone(), survey_hash, methodology_id, credit_qty, beneficiary),
);
```

**`dispute` and `resolve_dispute`**:

**`dispute(survey_hash)`**:
- Check survey exists, not resolved
- Set `disputed = true`
- Emit `("mrvo","disp")`

**`resolve_dispute(survey_hash, outcome, slashed_oracles[])`**:
- Admin only
- Check survey exists, not resolved
- Set `resolved = true`
- If outcome=fraud, slash each oracle in `slashed_oracles[]`
- Emit `("mrvo","resd")`

**Tests to add**:

- `test_submit_survey_creates_proposal` — Full flow: register oracles, set threshold, register polygon, submit survey, verify proposal created via mocked cross-contract
- `test_dispute_flow` — Submit, dispute, resolve
- `test_submit_while_paused` — `#[should_panic]`
- `test_submit_below_threshold_rejected` — Only 1 sig when threshold=2
- `test_submit_invalid_oracle_rejected` — Unknown pubkey in sigs
- `test_duplicate_survey_rejected` — Same hash submitted twice

**Verification**:

```bash
cargo test -p mrv-oracle
# 12+ tests pass
```

---

#### Day 7 — Approval Governance: Stakeholder Management

**Prompt for agent**: Implement stakeholder registration, role management, weighted voting configuration.

**Files to create**: `contracts/approval-gov/src/{lib,storage,types,errors,test}.rs`  
**Prerequisite**: Day 3 done.

**`contracts/approval-gov/src/types.rs`**:

```rust
#[derive(Clone, Debug)]
#[contracttype]
pub enum StakeholderRole {
    LeadEcologist = 0,
    PeerEcologist = 1,
    LocalCommunityRep = 2,
    IndependentAuditor = 3,
    MethodologyExpert = 4,
    RegulatoryObserver = 5,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Stakeholder {
    pub addr: Address,
    pub role: StakeholderRole,
    pub weight: u32,
    pub has_veto: bool,
    pub active: bool,
    pub registered_at: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub enum ProposalState {
    Draft = 0,
    Voting = 1,
    Approved = 2,
    Rejected = 3,
    Cancelled = 4,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Vote {
    pub voter: Address,
    pub approve: bool,
    pub weight: u32,
    pub comment_hash: BytesN<32>,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
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
    pub state: ProposalState,
    pub votes: Vec<Vote>,
    pub community_veto: bool,
    pub weighted_total_approve: u32,
    pub weighted_total_reject: u32,
}
```

**`contracts/approval-gov/src/errors.rs`**:

```rust
#[contracterror]
pub enum ApprovalError {
    Unauthorized = 1,
    StakeholderAlreadyRegistered = 2,
    StakeholderNotFound = 3,
    ProposalNotFound = 4,
    ProposalNotVoting = 5,
    ProposalAlreadyClosed = 6,
    VoterNotStakeholder = 7,
    VoteAlreadyCast = 8,
    CommunityVetoActivated = 9,
    ThresholdNotMet = 10,
    InvalidWeight = 11,
    VotingPeriodExpired = 12,
    VetoPowerRequired = 13,
    InvalidRole = 14,
}
```

**`contracts/approval-gov/src/storage.rs`**:

Singleton keys: `Admin`, `RecT` (bdc-token addr), `MrvO` (mrv-oracle addr), `MinW` (min weight threshold u32), `VPer` (voting period seconds u64), `PrCN` (proposal counter u64).

Per-stakeholder: prefix `0x10` + address bytes → `Stakeholder`.  
Per-proposal: prefix `0x20` + proposal_id_be_bytes → `Proposal`.  
Per-vote key: prefix `0x30` + proposal_id bytes + voter bytes → `Vote` (or embed in proposal).

**`contracts/approval-gov/src/lib.rs`**:

Functions to implement:

| Function | Description |
|----------|-------------|
| `__constructor(admin, min_weight, voting_period_secs)` | Init |
| `admin()` | Read admin |
| `transfer_admin(new)` | Both auth |
| `set_bdc_token(addr)` | Admin |
| `set_mrv_oracle(addr)` | Admin |
| `register_stakeholder(addr, role, weight, has_veto)` | Admin, check not duplicate, validate role |
| `remove_stakeholder(addr)` | Admin, set active=false |
| `get_stakeholder(addr)` | Check exists, return |
| `stakeholder_count()` | Return count |
| `set_min_threshold(min_weight)` | Admin |
| `min_threshold()` | Read |
| `set_voting_period(secs)` | Admin |
| `voting_period()` | Read |

Leave `propose`, `vote`, `veto`, `on_approved`, `close_proposal` as stubs.

**`contracts/approval-gov/src/test.rs`**:

Tests:
- `test_register_stakeholder` — Fields match, active=true
- `test_register_all_roles` — Register one of each role
- `test_remove_stakeholder` — active=false
- `test_set_threshold` — Read back
- `test_set_voting_period` — Read back
- `test_duplicate_stakeholder_rejected` — `#[should_panic]`
- `test_get_nonexistent_stakeholder` — `#[should_panic]`

**Verification**:

```bash
cargo test -p approval-gov
# 7+ tests pass
```

---

#### Day 8 — Approval Governance: Proposal & Voting Engine

**Prompt for agent**: Implement proposal creation, weighted voting, community veto, approval threshold checks.

**Files to modify**: `contracts/approval-gov/src/lib.rs` (replace stubs), test.rs  
**Prerequisite**: Day 7 done.

**`propose(polygon_id, survey_hash, methodology_id, credit_qty, beneficiary) -> u64`**:
1. Increment counter
2. Create `Proposal { state: Voting, voting_deadline: now + VPer, votes: vec![], ... }`
3. Emit `("gov","prop")`
4. Return proposal_id

**`vote(proposal_id, approve, comment_hash)`**:
1. `env.invoker().require_auth()`
2. Check proposal exists, state == Voting, deadline not passed
3. Check voter is registered stakeholder and active
4. Check voter hasn't already voted
5. Create `Vote`, append to proposal.votes
6. Update `weighted_total_approve` or `weighted_total_reject`
7. Emit `("gov","vote")`
8. After vote: check if `approval threshold` met or `community veto` triggered

**`veto(proposal_id)`**:
1. `env.invoker().require_auth()`
2. Check proposal exists, state == Voting
3. Check voter is stakeholder with `has_veto == true`
4. Set `community_veto = true`
5. Set `state = Rejected`
6. Emit `("gov","veto")`

**`on_approved(proposal_id)`**:
1. Check proposal exists, state == Approved
2. Cross-call to bdc-token.mint() with proposal params
3. Emit `("gov","appr")`

**`close_proposal(proposal_id)`**:
1. Admin or after deadline
2. If threshold met && !veto → state = Approved, call `on_approved`
3. Else → state = Rejected

**Threshold check logic**:
```rust
fn check_approval(env: &Env, proposal: &mut Proposal) {
    let min = read_min_threshold(env);
    if proposal.community_veto {
        proposal.state = ProposalState::Rejected;
        return;
    }
    if proposal.weighted_total_approve >= min {
        proposal.state = ProposalState::Approved;
        // cross-call to bdc-token.mint()
    } else if proposal.weighted_total_reject >= min {
        proposal.state = ProposalState::Rejected;
    }
    // else still voting
}
```

**Tests**:

| Test | Setup | Expected |
|------|-------|----------|
| `test_propose_creates_proposal` | Propose with valid params | state=Voting, deadline set |
| `test_vote_approve` | Stakeholder votes yes | weighted_total_approve incremented |
| `test_community_veto` | Community rep vetoes | state=Rejected, veto=true |
| `test_threshold_met_triggers_approval` | Enough weighted yes votes | state=Approved, mint called |
| `test_double_vote_rejected` | Same voter votes twice | `#[should_panic]` |
| `test_non_stakeholder_cannot_vote` | Random address votes | `#[should_panic]` |
| `test_vote_after_deadline` | Vote after period expired | `#[should_panic]` |
| `test_veto_only_for_community_role` | Non-veto stakeholder calls veto | `#[should_panic]` |

**Verification**:

```bash
cargo test -p approval-gov
# 15+ tests pass
```

---

#### Day 9 — Retirement Registry: Core Retirement

**Prompt for agent**: Implement batch retirement with polygon anchoring, receipt storage, token burning.

**Files to create**: `contracts/retirement/src/{lib,storage,types,errors,test}.rs`  
**Prerequisite**: Day 3 done (bdc-token burn works).

**`contracts/retirement/src/types.rs`**:

```rust
#[contracttype]
pub struct ClaimData {
    pub period_start: u64,
    pub period_end: u64,
    pub purpose: Bytes,
    pub jurisdiction: Bytes,
}

#[contracttype]
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

**`contracts/retirement/src/errors.rs`**:

```rust
#[contracterror]
pub enum RetirementError {
    Unauthorized = 1,
    ReceiptNotFound = 2,
    TokenAlreadyRetired = 3,
    InvalidToken = 4,
    PolygonMismatch = 5,
    EmptyTokenList = 6,
}
```

**`contracts/retirement/src/storage.rs`**:

Singleton keys: `Admin`, `RecT` (bdc-token addr), `RcCN` (receipt counter u64).

Per-receipt: prefix `0x10` + receipt_id bytes → `RetirementReceipt`.  
Per-token retired flag: prefix `0x20` + token_id_be_bytes → `bool`.

**`contracts/retirement/src/lib.rs`**:

**`retire(token_ids, polygon_id, claim_data) -> BytesN<32>`**:

1. `env.invoker().require_auth()`
2. Validate `token_ids` not empty
3. For each `token_id`:
   - Check not already retired (via `is_retired` flag), else panic `TokenAlreadyRetired`
4. For each `token_id`:
   - Cross-call to `bdc-token.burn()` (env.invoker, token_id)
   - Write retired flag
5. Compute merkle root from `token_ids`
6. Generate `receipt_id = sha256(polygon_id + retirer + timestamp + token_ids_hash)`
7. Build `RetirementReceipt`, store
8. Emit `("retr","done")` with `(receipt_id, polygon_id, token_count, total_credits)`
9. Return `receipt_id`

**`get_receipt(receipt_id) -> RetirementReceipt`**:
- Check exists, return

**`verify_retirement(token_id) -> bool`**:
- Check retired flag

**`verify_claim(polygon_id, claim_period_start, claim_period_end, retirer) -> bool`**:
- Scan receipts for matching polygon + period + retirer
- Return true if at least one match

**`is_token_retired(token_id) -> bool`**:
- Check flag

**`set_bdc_token(addr)`** — admin  
**`bdc_token()`** — read

**`contracts/retirement/src/test.rs`**:

Tests:
- `test_retire_tokens` — Retire 3 tokens, verify receipt fields, verify is_retired
- `test_retire_with_polygon_binding` — polygon_id stored in receipt
- `test_double_retire_rejected` — `#[should_panic(expected = "TokenAlreadyRetired")]`
- `test_get_nonexistent_receipt` — `#[should_panic(expected = "ReceiptNotFound")]`
- `test_empty_token_list_rejected` — `#[should_panic]`
- `test_verify_retirement` — Returns true after retire

**Verification**:

```bash
cargo test -p retirement
# 6+ tests pass
```

---

#### Day 10 — Retirement Registry: Merkle Proofs & Polygon Geometry

**Prompt for agent**: Implement Merkle tree for batch proofs, polygon containment verification, and prove_claim.

**Files to modify**: `contracts/retirement/src/lib.rs`, new `contracts/retirement/src/merkle.rs`, new `contracts/retirement/src/geometry.rs`, test.rs  
**Prerequisite**: Day 9 done.

**`contracts/retirement/src/merkle.rs`**:

Implement a binary Merkle tree:
- Leaves: `SHA256(token_id_be_bytes)`
- Internal nodes: `SHA256(left_hash || right_hash)`
- Duplicate odd node carries up

Functions:
- `compute_root(env, token_ids: &Vec<u64>) -> BytesN<32>`
- `generate_proof(env, token_ids, leaf_index) -> Vec<BytesN<32>>`
- `verify(env, root, proof, leaf, leaf_index) -> bool`

**`contracts/retirement/src/geometry.rs`**:

**Point-in-polygon using ray-casting algorithm**:

```rust
pub fn point_in_polygon(
    point: (i64, i64),          // (lat * 1_000_000, lon * 1_000_000)
    polygon: Vec<(i64, i64)>,   // vertices in fixed-point
) -> bool {
    let mut inside = false;
    let n = polygon.len();
    let mut j = n - 1;
    for i in 0..n {
        if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1))
            && (point.0 < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1)
                / (polygon[j].1 - polygon[i].1) + polygon[i].0)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}
```

**`prove_claim(wallet, period_start, period_end) -> (root, proof, leaf_index)`**:
- Scan receipts for matching retirer + period
- Generate merkle proof for the first matching receipt
- Return proof data (stub that returns empty for now, full implementation in Phase 3)

**`prove_polygon_containment(token_id, polygon_id) -> bool`**:
- Look up token's polygon_id from its metadata (via bdc-token cross-call)
- Verify it matches the given polygon_id

**Tests to add**:

- `test_merkle_root_deterministic` — Same input = same root
- `test_merkle_proof_verify` — Generate proof, verify against root
- `test_point_in_polygon_simple` — Square polygon, point inside
- `test_point_in_polygon_outside` — Point outside
- `test_prove_claim_returns_data` — Stub returns expected format

**Verification**:

```bash
cargo test -p retirement
# 10+ tests pass
```

---

#### Day 11 — Marketplace: Order Book

**Prompt for agent**: Implement `place_order`, `cancel_order`, `get_order` with price-time priority ordering and biome/vintage filters.

**Files to create**: `contracts/marketplace/src/{lib,storage,types,errors,order_book,test}.rs`  
**Prerequisite**: Day 3 done.

**`contracts/marketplace/src/types.rs`**:

```rust
#[contracttype]
pub enum OrderSide { Buy = 0, Sell = 1 }

#[contracttype]
pub enum OrderRestriction { None = 0, FillOrKill = 1, ImmediateOrCancel = 2 }

#[contracttype]
pub enum OrderStatus { Open = 0, Filled = 1, Cancelled = 2 }

#[contracttype]
pub struct Order {
    pub order_id: u64,
    pub trader: Address,
    pub side: OrderSide,
    pub price: i128,
    pub initial_qty: u64,
    pub remaining_qty: u64,
    pub timestamp: u64,
    pub restrictions: OrderRestriction,
    pub biome_filter: Option<u8>,
    pub vintage_filter: Option<u16>,
    pub status: OrderStatus,
}
```

**`contracts/marketplace/src/errors.rs`**:

```rust
#[contracterror]
pub enum MarketError {
    Unauthorized = 1,
    OrderNotFound = 2,
    OrderFilled = 3,
    PriceMismatch = 4,
    InsufficientBalance = 5,
    FeeCapExceeded = 6,
    BiomeMismatch = 7,
    VintageMismatch = 8,
    InvalidQuantity = 9,
}
```

**`contracts/marketplace/src/storage.rs`**:

Singleton keys: `Admin`, `FeeRt` (u16, default 25), `OrCN` (order counter u64), `RecT`, `USDC`, `FVal` (fee vault Address).

Per-order: prefix `0x10` + order_id_be_bytes → persistent `Order`.

**`contracts/marketplace/src/order_book.rs`**:

Functions:
- `buy_orders(env) -> Vec<Order>` — scan all orders, filter side=Buy + status=Open + remaining>0, sort price desc then timestamp asc
- `sell_orders(env) -> Vec<Order>` — same but price asc
- `best_bid(env) -> Option<Order>` — first of buy_orders
- `best_ask(env) -> Option<Order>` — first of sell_orders
- `best_bid_for_biome(env, biome) -> Option<Order>` — filtered by biome
- `best_ask_for_biome(env, biome) -> Option<Order>` — filtered by biome

Use a simple Vec-based insertion sort (gas-inefficient but correct for MVP).

**`contracts/marketplace/src/lib.rs`**:

| Function | Description |
|----------|-------------|
| `__constructor(admin)` | Store admin, fee_rate=25, counter=0 |
| `admin()`, `set_fee_rate(rate)`, `fee_rate()` | Fee capped at 100 bps |
| `set_bdc_token(id)`, `set_usdc_token(id)`, `set_fee_vault(addr)` | Admin-only |
| `place_order(side, price, qty, restrictions, biome_filter, vintage_filter) -> u64` | Increment counter, create Order, store, emit `("mkt","plac")` |
| `cancel_order(order_id)` | Check exists, caller owns it, status=Open, set Cancelled, emit |
| `get_order(order_id)` | Check exists, return |
| `get_best_bid()`, `get_best_ask()` | Delegate to order_book |
| `get_best_bid_for_biome(biome)`, `get_best_ask_for_biome(biome)` | Filtered queries |
| `get_buy_orders()`, `get_sell_orders()` | Return full sorted lists |

**`contracts/marketplace/src/test.rs`**:

Tests:
- `test_place_buy_order` — Returns id=1, fields match
- `test_place_sell_order` — side=Sell
- `test_cancel_order` — status changes to Cancelled
- `test_best_bid_ask` — After placing both sides, verify best bid/ask
- `test_biome_filtered_orders` — Only matching biome returned
- `test_cancel_nonexistent` — `#[should_panic(expected = "OrderNotFound")]`
- `test_price_time_priority` — Multiple buys at different prices, best is highest
- `test_fok_immediate_cancel` — FOK with insufficient liquidity → cancelled

**Verification**:

```bash
cargo test -p marketplace
# 8+ tests pass
```

---

#### Day 12 — Marketplace: Matching Engine

**Prompt for agent**: Implement `match_orders` (atomic match with BDC transfer, yUSDC settlement, fee deduction) and `auto_match`.

**Files to modify**: `contracts/marketplace/src/lib.rs`, test.rs  
**Prerequisite**: Day 11 done.

**Cross-contract call patterns**:

Add to `lib.rs` — inline invoke helpers:

```rust
fn transfer_bdc(env: &Env, bdc_id: &Address, from: &Address, to: &Address, token_id: u64) {
    env.invoke_contract(bdc_id, &Symbol::short("tran"), (from.clone(), to.clone(), token_id));
}

fn transfer_usdc(env: &Env, usdc_id: &Address, from: &Address, to: &Address, amount: i128) {
    env.invoke_contract(usdc_id, &Symbol::short("xfer"), (from.clone(), to.clone(), amount));
}
```

**`match_orders(buy_id, sell_id) -> (u64, i128, i128)`**:

Returns `(fill_qty, fill_price, fee)`.

1. Load both orders, validate: both exist, both Open, remaining > 0, buy side=Buy sell side=Sell
2. Price check: `buy.price >= sell.price` else panic `PriceMismatch`
3. Biome filter check: if both have filters, they must match
4. Vintage filter check: if both have filters, they must match
5. Calculate `fill_qty = min(buy.remaining, sell.remaining)`, `fill_price = sell.price`
6. Calculate `fill_value = fill_price * fill_qty`, `fee = fill_value * fee_rate / 10000`
7. Transfer BDC tokens from seller to buyer (iterate token_ids)
8. Transfer yUSDC from buyer to seller (amount = fill_value - fee)
9. Transfer yUSDC from buyer to fee_vault (amount = fee)
10. Update both orders' `remaining_qty`, set status to Filled if 0
11. Emit `("mkt","matc")` with `(buy_id, sell_id, fill_qty, fill_price, fee)`

**`auto_match() -> u32`**:

Loop: get best_bid and best_ask. If bid.price >= ask.price, call match_orders. Repeat until spread opens or no orders. Respect biome filters. Return count of matches.

**Tests**:

- `test_match_orders_basic` — Buy 10 @ $1, Sell 10 @ $1 → fill 10
- `test_match_partial_fill` — Buy 10, Sell 5 → fill 5, buy has 5 remaining
- `test_price_mismatch_rejected` — Buy $0.90, Sell $1.00 → panic
- `test_biome_filter_match` — Both have same biome filter → match
- `test_biome_mismatch_rejected` — Different biomes → panic
- `test_auto_match_multiple` — 3 cross-book orders → all matched
- `test_fee_deduction` — Verify fee math at 25 bps
- `test_fok_enforcement` — FOK order with insufficient fill → cancelled

**Verification**:

```bash
cargo test -p marketplace
# 16+ tests pass
```

---

### Week 2 — Integration, DevOps & Polish

---

#### Day 13 — Integration Tests

**Prompt for agent**: Write end-to-end Soroban sandbox tests exercising cross-contract flows across all 5 contracts.

**Files to modify**: `tests/integration/src/lib.rs`  
**Prerequisite**: All 5 contracts compile with passing unit tests.

**Test structure**:

Wrap all tests in `#[cfg(test)] mod test { ... }`.

Each test deploys all 5 contracts in the same `Env`, links them, and exercises a full lifecycle.

**Test: `test_full_lifecycle`**:

1. Create `Env::default()`, `env.mock_all_auths()`
2. Create addresses: `admin`, `project_dev`, `buyer`, `ecologist`, `community_rep`, `auditor`, `oracle1_pk`, `oracle2_pk`, `oracle3_pk`, `polygon_id`
3. Deploy BDC Token → `__constructor(&admin)`
4. Deploy MRV Oracle → `__constructor(&admin, &bdc_id, 2u32, 3u32)`
5. Link: `mrv_oracle.set_bdc_token(&bdc_id)`, `bdc_token.authorize_minter(&approval_gov_id)`
6. Deploy Approval Gov → `__constructor(&admin, 6u32, 604800u64)` (threshold=6, 7-day voting)
7. Link: `approval_gov.set_bdc_token(&bdc_id)`, `approval_gov.set_mrv_oracle(&mrv_id)`
8. Register 3 oracles of different types, set threshold 2-of-3
9. Register polygon with bounding box
10. Register stakeholders (ecologist weight=3, community rep weight=2 veto=true, auditor weight=1)
11. Submit survey → triggers proposal creation
12. Vote approve from all stakeholders → threshold met → proposal approved
13. Verify BDC token minted (`total_supply() == credit_qty`)
14. Deploy Marketplace → `__constructor(&admin, &bdc_id, &usdc_id, &fee_vault, 25u16)`
15. Place sell order, place buy order, match them
16. Deploy Retirement → `__constructor(&admin, &bdc_id)`
17. Retire BDC tokens with polygon_id → verify receipt, verify `is_retired(token_id)`
18. Verify claim

**Test: `test_unauthorized_mint_rejected`**:

- Deploy MRV oracle, do NOT call `authorize_minter`
- Attempt to submit survey → should fail

**Test: `test_community_veto_rejects_proposal`**:

- Full setup, community rep vetoes → proposal rejected, no tokens minted

**Test: `test_double_retire_rejected`**:

- Mint BDC, retire it, retire again → panic

**Verification**:

```bash
cargo test -p integration-tests
# 4+ integration tests pass
```

---

#### Day 14 — Scripts & CI/CD

**Prompt for agent**: Create deployment scripts (sandbox + testnet), test runner, finalize CI.

**Files to create**: `scripts/{setup,deploy,test}.sh`  
**Prerequisite**: All previous days done.

**`scripts/setup.sh`**:

```bash
#!/usr/bin/env bash
set -euo pipefail

# 1. Check prerequisites (cargo, soroban CLI)
# 2. Build all contracts (cargo build --release)
# 3. Deploy each contract to local sandbox
# 4. Initialize cross-contract references
# 5. Print deployed contract IDs and summary
```

Key operations:
```bash
BDC_TOKEN_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/bdc_token.wasm \
  --network local --source admin)

soroban contract invoke --id "$BDC_TOKEN_ID" --fn __constructor --arg "$ADMIN"
```

Full deployment order:
1. Deploy BDC Token → init
2. Deploy MRV Oracle → init (with bdc-token id)
3. Deploy Approval Gov → init (with bdc-token id, mrv-oracle id)
4. Authorize approval-gov as minter on bdc-token
5. Deploy Retirement → init (with bdc-token id)
6. Deploy Marketplace → init (with bdc-token id, usdc id)
7. Print all IDs

**`scripts/deploy.sh`**:

```bash
#!/usr/bin/env bash
set -euo pipefail
NETWORK="${1:-testnet}"
# Build WASM, deploy all 5 contracts, init, print IDs
```

**`scripts/test.sh`**:

```bash
#!/usr/bin/env bash
set -euo pipefail
cargo test --all-features
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo doc --no-deps --document-private-items
```

Make all scripts executable: `chmod +x scripts/*.sh`

**Verification**:

```bash
cargo test --all-features  # all unit + integration tests pass
cargo clippy --all-targets # no warnings
cargo fmt --check          # properly formatted
```

---

#### Day 15 — Documentation

**Prompt for agent**: Write architecture, methodology, polygon spec, and oracle network documentation.

**Files to create**: `docs/{architecture,methodology,polygon-spec,oracle-spec}.md`, `CONTRIBUTING.md`, `SECURITY.md`  
**Prerequisite**: All previous days done.

**`docs/architecture.md`**:

Cover:
- Layer diagram (contract → Soroban → Stellar)
- Crate dependency graph (bdc-token ← mrv-oracle ← approval-gov, etc.)
- Cross-contract call patterns (which contract calls which and when)
- Storage architecture (instance vs persistent keyspace conventions)
- Deployment topology (contract initialization order)

**`docs/methodology.md`**:

Document the Biodiversity Score Index (BSI):
```
BSI = 0.30 × SR_norm + 0.25 × SDI_norm + 0.20 × CC_norm + 0.15 × IUCN_norm + 0.10 × FQ_norm

BDC_credits = (BSI_current - BSI_baseline) × polygon_area_ha × quality_multiplier
```

Include:
- Full formula with variable definitions
- Numerical examples (e.g., baseline 0.28 → current 0.64 on 4,500 ha = 1,620 credits)
- Quality multiplier table (standard=1.0, IUCN priority=1.2, FPIC=1.5, restored=0.8)
- Methodology versioning scheme (BDCX-TF-v1.0, BDCX-MG-v1.0, etc.)
- Cross-validation tolerances between oracle types

**`docs/polygon-spec.md`**:

Cover:
- On-chain bounding box vs off-chain full GeoJSON on IPFS
- Coordinate encoding (fixed-point i64 × 1,000,000)
- Point-in-polygon ray-casting algorithm pseudocode
- Polygon lifecycle (registered → active → closed)
- Minimum area requirements
- Biome classification scheme

**`docs/oracle-spec.md`**:

Cover:
- Oracle network topology diagram
- Threshold voting (N-of-M) specification
- Signature verification flow
- IPFS content addressing and CID verification
- Cross-validation tolerance between oracle types
- Dispute resolution flow
- Slashing conditions and amounts

**`CONTRIBUTING.md`**:

Sections:
- Quick start (build, test)
- Project structure
- PR process (fork, branch, tests must pass, clippy clean, fmt applied)
- Coding standards (#![no_std], require_auth, typed errors, events)
- How to add a new methodology

**`SECURITY.md`**:

```
# Security Policy

## Reporting
Report to security@bdcx.dev. Do NOT open public issues.

## Scope
Smart contract logic, MRV oracle sig verification, cross-contract auth, approval governance.

## Bug Bounty
TBD
```

**Verification**:

```bash
cargo doc --no-deps
```

---

#### Day 16 — Frontend Skeleton + Polish

**Prompt for agent**: Scaffold React + TypeScript + Vite project with Soroban SDK and MapLibre GL JS. Create 3 read-only pages with geo-spatial polygon viewer.

**Files to create**: All under `frontend/`

```
frontend/package.json
frontend/tsconfig.json
frontend/vite.config.ts
frontend/index.html
frontend/src/main.tsx
frontend/src/App.tsx
frontend/src/components/Header.tsx
frontend/src/components/PolygonMap.tsx
frontend/src/pages/DashboardPage.tsx
frontend/src/pages/MarketplacePage.tsx
frontend/src/pages/PortfolioPage.tsx
frontend/src/hooks/useSoroban.ts
frontend/src/hooks/usePolygons.ts
```

**`package.json`**:

```json
{
  "name": "bdcx-frontend",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": { "dev": "vite", "build": "tsc && vite build" },
  "dependencies": {
    "@stellar/stellar-sdk": "^12.0.0",
    "maplibre-gl": "^4.0.0",
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "react-router-dom": "^7.0.0"
  },
  "devDependencies": {
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0",
    "@types/maplibre-gl": "^4.0.0",
    "typescript": "^5.6.0",
    "vite": "^6.0.0",
    "@vitejs/plugin-react": "^4.0.0"
  }
}
```

**`App.tsx`**:

React Router with 3 routes: `/` (Dashboard), `/marketplace`, `/portfolio`.

**`components/PolygonMap.tsx`**:

MapLibre GL JS component that renders habitat polygon bounding boxes as rectangles on an interactive map. Props: `polygons: Array<{id, bbox, biome, credits}>`. Show tooltip on hover with polygon stats.

**`pages/DashboardPage.tsx`**:

- 4 stat cards (Total BDCs Minted, Active Polygons, Active Orders, BDCs Retired) — all showing placeholder "—"
- PolygonMap component showing all registered polygons
- Recent activity feed (last 5 events)

**`pages/MarketplacePage.tsx`**:

- Two columns: Buy Orders (left), Sell Orders (right)
- Biome filter dropdown
- Best bid/ask display
- Each shows "No orders yet." placeholder

**`pages/PortfolioPage.tsx`**:

- Two sections: BDC Holdings, Retirement History
- Polygon map showing polygons where user has retired credits
- Claim verification tool (enter claim ID → show verification result)

**`hooks/useSoroban.ts`**:

Returns `{ connected, contractId, rpcUrl }`. Reads `VITE_RPC_URL` and `VITE_CONTRACT_ID` from env.

**`hooks/usePolygons.ts`**:

Returns mock polygon data for read-only display. Later replaced with Soroban RPC queries.

**Verification**:

```bash
cd frontend && npm install && npm run dev
# http://localhost:5173 shows 3 pages with navigation + interactive map
```

---

## 📊 Deliverable Summary

| Metric | Target |
|--------|--------|
| Lines of Rust code | ~10,000–14,000 |
| Smart contracts | 5 (all functional) |
| Unit tests | 180+ |
| Integration tests | 4+ |
| CI checks | 6 (check, test, clippy, fmt, integration, doc) |
| Documentation pages | 6 (`architecture`, `methodology`, `polygon-spec`, `oracle-spec`, `CONTRIBUTING`, `SECURITY`) |
| Frontend pages | 3 (Dashboard with geo-map, Marketplace, Portfolio) |
| Deployment scripts | 3 (sandbox, testnet, test runner) |
| GitHub templates | 3 (bug, feature, PR) |

---

## 🔮 What Remains (the 45%)

| Area | What's Left |
|------|-------------|
| CfD Liquidation Engine | Full margin-call automation, liquidation auction, bad-debt handling |
| Frontend | Trading UI, wallet connect, transaction submission, real-time updates |
| Fuzz Testing | Property-based fuzzing for all contracts |
| Governance / DAO | Token-based voting, parameter voting, upgrade proposals |
| ZK Proofs | Full merkle proof verification in retirement contract |
| Cross-Chain Bridges | Celo, Polygon, Regen Network ↔ BDCX bridge contracts |
| Indexer | PostgreSQL event sink, GraphQL API |
| Production Hardening | Multi-sig admin, emergency drills, audit remediation |
| Methodology Expansion | Coral reef, wetland, grassland methodologies |
| Automated MRV | Real-time sensor stream integration |
| AI Fraud Detection | Oracle behavior scoring, anomaly detection |
| Biodiversity Derivatives | Options, futures on BDC price indices |
| Insurance Products | Conservation outcome insurance |
| Bug Bounty Program | Setup, scope, rewards |
| Mainnet Deployment | Actual Stellar pubnet deployment + verification |

---

> **This plan gives contributors a solid, working codebase with clear extension points. The first 55% is the hardest — architecture, contracts, and testing patterns. The remaining 45% is largely additive and well-scoped.**
