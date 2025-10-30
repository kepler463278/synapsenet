# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in SynapseNet, please report it responsibly:

**GitHub Security Advisories**: https://github.com/kepler463278/synapsenet/security/advisories/new

**Email** (for sensitive issues): Kepler3124@proton.me

**PGP Key**: Available upon request

## Scope

Security issues include:
- Cryptographic vulnerabilities (signature forgery, key leakage)
- P2P network attacks (eclipse, sybil, DDoS)
- Storage vulnerabilities (SQL injection, data corruption)
- PoE gaming (spam, reward manipulation)
- Privacy leaks (deanonymization, metadata exposure)

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 7 days
- **Fix timeline**: Depends on severity (critical: 7-14 days)
- **Public disclosure**: After fix is released

## Security Best Practices

### For Node Operators

1. **Key Management**
   - Store `node.key` securely (encrypted filesystem recommended)
   - Never share private keys
   - Backup keys offline

2. **Network Security**
   - Use firewall rules to limit P2P exposure
   - Monitor peer reputation scores
   - Rate-limit incoming connections

3. **Data Integrity**
   - Verify grain signatures before indexing
   - Regular database backups
   - Check storage checksums

### For Developers

1. **Code Review**
   - All crypto code requires peer review
   - Use `cargo audit` for dependency vulnerabilities
   - Run `cargo clippy` for common issues

2. **Testing**
   - Write tests for security-critical paths
   - Fuzz test parsers and deserializers
   - E2E tests for P2P scenarios

3. **Dependencies**
   - Pin dependency versions
   - Review security advisories
   - Minimize external dependencies

## Known Limitations (v0.1)

- **No Byzantine fault tolerance**: Assumes honest majority
- **No Sybil resistance**: Reputation system is basic
- **No formal verification**: Crypto primitives not formally verified
- **Local-only mode**: P2P not fully implemented yet

## Cryptographic Primitives

- **Signatures**: ed25519-dalek (EdDSA)
- **Hashing**: blake3
- **Random**: OsRng (OS-provided CSPRNG)

## Disclosure Policy

We follow coordinated disclosure:
1. Reporter notifies us privately
2. We confirm and develop fix
3. Fix is released
4. Public disclosure after 90 days or fix release (whichever is sooner)

## Bug Bounty

Not available yet. Planned for post-mainnet launch.

## Contact

- **Security issues**: https://github.com/kepler463278/synapsenet/security/advisories/new
- **Email**: Kepler3124@proton.me
- **GitHub**: https://github.com/kepler463278/synapsenet

---

*Security is a community effort. Thank you for helping keep SynapseNet safe.*
