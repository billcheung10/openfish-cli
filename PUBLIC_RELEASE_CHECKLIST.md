# Openfish CLI Public Release Checklist

## Scope

This package may include:

- CLI commands for registration/setup guidance, wallet configuration, markets,
  order books, balances, orders, cancellations, status checks, and JSON output.
- The Rust SDK required by those CLI commands.
- Public examples and tests that use dummy credentials only.

This package must not include:

- Production private keys, API secrets, passphrases, seed phrases, or SSH keys.
- Admin/backend/deploy server code.
- Internal-only endpoints or operational scripts.
- Test wallets with real funds.
- Infrastructure hostnames, private IPs, PEM file names, or deployment paths.

## Required Commands

```bash
python3 scripts/public_safety_scan.py
cargo fmt --all --check
cargo build
cargo test --workspace
```

## OpenClaw Contract

The public CLI must support non-interactive agent use:

- all read commands support `-o json`
- write/trade commands expose explicit flags and fail closed
- secrets are loaded from environment/config, not printed into chat
- dry-run or readiness paths are available before live trading

## First Public Repo Tasks

- Create `billcheung10/openfish-cli` as a public repository.
- Push this directory as repository root, not as a subdirectory.
- Enable GitHub Actions.
- Tag the first release only after `cargo test --workspace --locked` passes.
- Update Homebrew checksums from release artifacts before documenting Homebrew as
  stable.
