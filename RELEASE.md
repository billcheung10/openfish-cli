# Release Process

This directory is designed to be published as the root of the public
`billcheung10/openfish-cli` repository.

## Preflight

```bash
python3 scripts/public_safety_scan.py
cargo fmt --all --check
cargo build --workspace --locked
cargo test --workspace --locked
```

## Tag Release

1. Update versions in:
   - `openfish-cli/Cargo.toml`
   - `openfish-client-sdk/Cargo.toml`
   - `openfish-cli/Formula/openfish.rb`
2. Commit the version bump.
3. Tag:

```bash
git tag v0.1.5
git push origin main --tags
```

The release workflow builds Linux and macOS archives and publishes
`checksums.txt`.

## Install Methods

Before the first tagged public release, the supported install path is source:

```bash
cargo install --path openfish-cli
```

After release artifacts exist, users can install with:

```bash
curl -sSL https://raw.githubusercontent.com/billcheung10/openfish-cli/main/openfish-cli/install.sh | sh
```

Homebrew formula checksums must be updated from the release artifacts before
advertising Homebrew as a stable install path.
