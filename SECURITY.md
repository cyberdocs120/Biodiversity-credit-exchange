# Security Policy

## Reporting a Vulnerability

**Do not open public GitHub issues for security vulnerabilities.**

Please report security issues to **security@bdcx.dev**. You should receive a response within 48 hours.

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Affected contracts or components
- Potential impact
- Any suggested remediation (optional)

## Scope

The following are in scope for security reports:

- Smart contract logic in all 5 BDCX contracts (`bdc-token`, `mrv-oracle`, `approval-gov`, `retirement`, `marketplace`)
- MRV oracle signature verification and threshold validation
- Cross-contract authorization and access control
- Approval governance voting integrity
- Retirement registry and merkle proof verification
- Marketplace matching engine and fee handling
- Upgrade and initialization logic

### Out of Scope

- Frontend application (UI, CSS, etc.)
- Off-chain infrastructure (IPFS, databases, etc.)
- Third-party dependencies (Soroban SDK, Stellar Core)
- Social engineering attacks

## Bug Bounty

Bug bounty program details are **TBD**. In the interim, critical vulnerabilities will be evaluated on a case-by-case basis for reward consideration.

## Safe Harbor

Any security research conducted in good faith, following this policy, is authorized. We will not take legal action against researchers who:

- Report vulnerabilities according to this policy
- Do not exploit vulnerabilities beyond what is necessary to demonstrate the issue
- Do not access or modify user data without explicit permission
- Act in good faith to improve the security of the protocol

## Disclosure Policy

We follow a responsible disclosure process:

1. Report received and acknowledged within 48 hours
2. Initial assessment within 5 business days
3. Fix developed and tested internally
4. Fix deployed to all affected networks
5. Public disclosure 30 days after fix deployment

## Contract Security Considerations

### Access Control

| Contract      | Admin Functions                                  |
|---------------|-------------------------------------------------|
| bdc-token     | `authorize_minter`, `revoke_minter`, `authorize_burner`, `set_metadata_uri` |
| mrv-oracle    | Oracle CRUD, polygon CRUD, threshold, pause/resume, dispute resolution |
| approval-gov  | Stakeholder CRUD, threshold, voting period, close proposal |
| retirement    | `set_bdc_token`                                 |
| marketplace   | Fee rate, fee vault, token addresses            |

### Known Limitations

1. **Single admin model** — All contracts use a single admin address. Multi-sig admin is planned for production.
2. **No upgrade mechanism** — Contracts are immutable after deployment. Future upgrades require new deployment and state migration.
3. **Gas limits** — Order book scan uses O(n) iteration. Large order books may hit Soroban gas limits.
4. **Merkle proof verification** — Uses SHA-256, not optimized for gas.
5. **No reentrancy guards** — Cross-contract calls should be reviewed for reentrancy risks.
