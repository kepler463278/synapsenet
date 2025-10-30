# Contributing to SynapseNet

Thank you for your interest in contributing to SynapseNet!

## Development Setup

### Prerequisites

- Rust (stable) - https://rustup.rs
- SQLite
- CMake
- pkg-config (Linux/macOS)

### Build

```bash
cargo build
```

### Test

```bash
cargo test
```

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## Code Style

- Follow Rust standard style (enforced by `rustfmt`)
- Write tests for new features
- Document public APIs
- Keep commits atomic and well-described

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linters
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## PR Guidelines

- **Title**: Clear and descriptive
- **Description**: Explain what and why
- **Tests**: Include tests for new functionality
- **Documentation**: Update docs if needed
- **Commits**: Keep them clean and atomic

## Code Review

All submissions require review. We use GitHub pull requests for this purpose.

## Testing

- Unit tests: `cargo test`
- E2E tests: `cargo test --test e2e`
- Benchmarks: `./scripts/bench.sh`

## Security

Report security vulnerabilities to security@synapsenet.org (see SECURITY.md)

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.

## Questions?

Open an issue or join our community chat.

---

*"Intelligence belongs to society. The center does not exist."*
