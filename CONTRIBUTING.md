# Contributing to shrimp-checker

Thank you for considering a contribution. This document describes how to get started.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain, 1.88+)

## Development setup

```bash
git clone https://github.com/wielorzeczownik/shrimp-checker.git
cd shrimp-checker
cargo run
```

## Before submitting a PR

Make sure these pass locally:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --all-targets --locked
```

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

Common prefixes:

| Prefix | When to use |
|--------|-------------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `chore:` | Maintenance, dependency updates |
| `refactor:` | Code change without behavior change |
| `docs:` | Documentation only |
| `style:` | Formatting, no logic change |
| `ci:` | CI/CD changes |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

## Pull requests

- Keep PRs focused on a single concern.
- Reference any related issue in the PR description.
- The CI `cargo-check` workflow must pass.

## Reporting bugs

Open an [issue](https://github.com/wielorzeczownik/shrimp-checker/issues) and include:
- What you did
- What you expected
- What actually happened
- Your platform (macOS, Windows, Linux)

> For security issues, please read [SECURITY.md](SECURITY.md) before opening a public issue.

## License

By contributing you agree that your changes will be licensed under the [MIT License](LICENSE).
