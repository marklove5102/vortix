# Contributing to Vortix

Thanks for your interest in contributing! 🎉

## Quick Start

```bash
git clone https://github.com/Harry-kp/vortix.git
cd vortix
cargo build
sudo cargo run
```

## Ways to Contribute

- 🐛 **Report bugs** — Open an issue with steps to reproduce
- 💡 **Suggest features** — Check the [roadmap](ROADMAP.md) first, then open an issue
- 📖 **Improve docs** — README, code comments, examples
- 🧪 **Add tests** — Unit tests, integration tests
- 🍎 **Linux support** — Help port macOS-specific code

## Development Workflow

1. Fork the repo
2. Create a feature branch: `git checkout -b feat/my-feature`
3. Make your changes
4. Run checks:
   ```bash
   cargo fmt        # Format code
   cargo clippy     # Lint
   cargo test       # Run tests
   ```
5. Commit with [conventional commits](https://www.conventionalcommits.org/):
   - `feat:` new feature
   - `fix:` bug fix
   - `docs:` documentation
   - `refactor:` code refactoring
6. Push and open a PR

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Keep functions small and focused
- Add doc comments for public APIs

## Testing

Vortix requires root for VPN operations. For testing:

```bash
# Run unit tests (no root needed)
cargo test

# Run with demo mode (masks sensitive data)
sudo cargo run -- --demo
```

## Questions?

Open a [discussion](https://github.com/Harry-kp/vortix/discussions) or reach out on [Twitter/X](https://twitter.com/harrykp007).
