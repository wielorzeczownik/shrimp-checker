# Contributing to shrimp-checker

Thank you for considering a contribution. This document covers everything you need to get started.

## Overview

A lightweight desktop GUI app that checks whether you are a shrimp – built with Rust and [Iced](https://github.com/iced-rs/iced).

## Project structure

```text
.
├── src/                 Rust source code
│   ├── main.rs          entry point
│   ├── app.rs           application logic (Iced)
│   ├── constants.rs     shared constants
│   └── i18n/            internationalization (en, pl)
├── scripts/
│   └── bump-version.sh  determines the next release version and bumps Cargo.toml
└── assets/              images and audio used by the app
```

## Development setup

```bash
git clone https://github.com/wielorzeczownik/shrimp-checker.git
cd shrimp-checker
cargo run
```

## Running checks locally

### With tools installed locally

```bash
# Rust
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo check --all-targets --locked
cargo audit

# Shell
shfmt --diff scripts/

# Markdown
markdownlint-cli2 "**/*.md"
```

### With Docker (no local installs required)

```bash
docker run --rm -v "$(pwd):/src" -w /src mvdan/shfmt --diff scripts/

docker run --rm -v "$(pwd):/workdir" davidanson/markdownlint-cli2 "**/*.md"
```

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

| Prefix      | When to use                         |
| ----------- | ----------------------------------- |
| `feat:`     | New feature or behavior             |
| `fix:`      | Bug fix                             |
| `chore:`    | Maintenance, dependency updates     |
| `refactor:` | Code change without behavior change |
| `docs:`     | Documentation only                  |
| `style:`    | Formatting, no logic change         |
| `ci:`       | CI/CD changes                       |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

Keep commits focused on a single concern. If a change touches both logic and tests, a single commit is fine – if it touches unrelated areas, split it.

## Pull requests

- Keep PRs focused on a single concern.
- Reference any related issue in the PR description.
- All CI checks must pass before merging.

## Reporting bugs

Open an [issue](https://github.com/wielorzeczownik/shrimp-checker/issues) and include:

- What you did
- What you expected
- What actually happened
- Your platform (macOS, Windows, Linux)

> For security issues, read [SECURITY.md](SECURITY.md) before opening a public issue.

## License

By contributing you agree that your changes will be licensed under the [MIT License](LICENSE).
