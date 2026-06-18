# Contributing to BDCX

Thank you for your interest in contributing to the Biodiversity Credit Exchange! 🌿

Please note that this project is released with a [Contributor Code of Conduct](./CODE_OF_CONDUCT.md). By participating in this project you agree to abide by its terms.

## Quick Start

```bash
# Build everything
cargo build --release

# Run all tests (unit + integration)
cargo test --all-features

# Lint
cargo clippy --all-targets -- -D warnings

# Format
cargo fmt --check
```

## Project Structure

```
├── contracts/
│   ├── bdc-token/        # SEP-41 token with polygon binding
│   ├── mrv-oracle/       # MRV oracle with N-of-M threshold voting
│   ├── approval-gov/     # Multi-stakeholder weighted voting governance
│   ├── retirement/       # Polygon-anchored retirement with merkle proofs
│   └── marketplace/      # Order book + matching engine + CfD
├── tests/
│   └── integration/      # Cross-contract integration tests
├── scripts/              # Deployment and test scripts
├── docs/                 # Architecture and methodology docs
└── frontend/             # React + TypeScript + MapLibre web app
```

## PR Process

1. **Fork** the repository
2. **Create a feature branch** from `main`
3. **Make your changes** following the coding standards below
4. **Run tests** — all must pass (`cargo test --all-features`)
5. **Lint** — zero clippy warnings (`cargo clippy --all-targets -- -D warnings`)
6. **Format** — must be clean (`cargo fmt --check`)
7. **Document** — update relevant docs if changing public interfaces
8. **Open a PR** against `main` with a clear description

### PR Checklist

- [ ] Tests added/updated and passing
- [ ] Clippy clean
- [ ] `cargo fmt` applied
- [ ] Documentation updated (if applicable)
- [ ] No `#![allow(...)]` unless justified in comment
- [ ] Events added/updated for state changes

## Coding Standards

### General

- All contracts are `#![no_std]` — no stdlib, no alloc (except `Vec` via Soroban)
- All public functions must include proper `require_auth()` checks
- All errors must use the typed `#[contracterror]` enum pattern
- All state-changing operations must emit events
- All storage keys must use the prefix convention (see docs/architecture.md)

### Smart Contract Pattern

```rust
#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn my_function(env: Env, param: Type) -> Result<Type, Error> {
        // 1. Auth check
        let caller = env.invoker();
        caller.require_auth();

        // 2. Validate inputs
        // 3. Read/write storage
        // 4. Emit event
        // 5. Return result
    }
}
```

### Naming Conventions

| Item              | Convention            | Example                    |
|-------------------|-----------------------|----------------------------|
| Contracts         | PascalCase            | `BdcTokenContract`         |
| Functions         | snake_case            | `register_oracle`          |
| Types/Enums       | PascalCase            | `OracleNode`, `OrderSide`  |
| Enum variants     | PascalCase            | `Buy`, `Sell`              |
| Errors            | PascalCase            | `OracleNotFound`           |
| Storage keys      | 4-char Symbol         | `Symbol::short("Admin")`   |
| Local variables   | snake_case            | `polygon_count`            |
| Constants         | SCREAMING_SNAKE_CASE  | `MAX_FEE_RATE`             |

### Testing Requirements

- Every function must have at least one test
- Happy path and at least one failure path
- Use `#[should_panic(expected = "...")]` for expected failures
- Use `env.mock_all_auths()` for integration tests
- Use descriptive test names: `test_<function>_<scenario>`

## How to Add a New Methodology

1. Add a new variant to the `Biome` enum in `bdc-token/src/types.rs`
2. Create a new methodology version string (e.g., `BDCX-CR-v1.0`)
3. Add biome-specific min/max area bounds in `mrv-oracle`
4. Add relevant oracle cross-validation tolerances in `docs/oracle-spec.md`
5. Update `docs/methodology.md` with the new BSI component weights
6. Add tests for the new biome in the relevant contracts

## Security

See `SECURITY.md` for our security policy and vulnerability reporting process.

## License

This project is licensed under the MIT License — see `LICENSE` for details.
