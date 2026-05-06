# Openfish CLI

Rust CLI for Openfish. Browse markets, inspect order books, manage wallets,
place orders, bridge FISH, and expose stable JSON output for scripts and
OpenClaw-generated trading agents.

> Warning: This is early, experimental trading software. Use at your own risk,
> start small, and verify every live action before approving it. APIs, commands,
> and behavior may change without notice.

## Install

### Supported Platforms

Source install works on any platform where the Rust toolchain and Openfish
dependencies build.

Release binaries are published for:

- Linux x86_64: `x86_64-unknown-linux-gnu`
- Linux ARM64: `aarch64-unknown-linux-gnu`
- macOS Intel: `x86_64-apple-darwin`
- macOS Apple Silicon: `aarch64-apple-darwin`
- Windows x86_64: `x86_64-pc-windows-msvc`

Windows binaries are published as `.zip` archives. Unix-like platforms are
published as `.tar.gz` archives.

### Build from source

Source install is always available and is the best path before a matching
binary release is published for your platform.

```bash
git clone https://github.com/billcheung10/openfish-cli
cd openfish-cli
cargo install --path openfish-cli
```

### Shell script

Installs the latest published release binary:

```bash
curl -sSL https://raw.githubusercontent.com/billcheung10/openfish-cli/main/openfish-cli/install.sh | sh
```

### Homebrew

Available after release checksums are published for the latest release:

```bash
brew tap billcheung10/openfish-cli https://github.com/billcheung10/openfish-cli
brew install openfish
```

### Windows

Native Windows binaries are published as `.zip` archives on GitHub Releases.
You can also build from source:

```powershell
git clone https://github.com/billcheung10/openfish-cli
cd openfish-cli
cargo install --path openfish-cli
```

## Update

If you installed a release binary, use:

```bash
openfish upgrade
```

`openfish upgrade` requires a published GitHub Release with a matching binary
and checksum for your platform.

If you installed from source, update from the repository:

```bash
cd openfish-cli
git pull
cargo install --path openfish-cli --force
```

## Uninstall

If installed with Cargo:

```bash
cargo uninstall openfish-cli
```

If installed from a release archive or shell installer, remove the installed
`openfish` binary from your `PATH`. On many macOS and Linux systems this is:

```bash
rm /usr/local/bin/openfish
```

On Windows, remove the directory containing `openfish.exe` from `PATH`.

Uninstalling the binary does not delete local wallet or API configuration. Only
remove the Openfish config directory if you intentionally want to wipe local
credentials and settings.

## Quick Start

```bash
# No wallet needed for read-only market research
openfish markets list --limit 5
openfish markets search "bitcoin"
openfish events list --tag crypto

# Check one market
openfish markets get bitcoin-above-100k

# JSON output for scripts and agents
openfish -o json markets list --limit 3
openfish -o json clob book TOKEN_ID
```

To trade, configure a wallet, register the wallet with Openfish, then deposit
FISH into the agent account:

```bash
openfish setup

# Or run the steps manually
openfish wallet create
openfish clob create-api-key --agent-env-file .env.agent
openfish clob account-status
openfish bridge deposit $(openfish wallet address)
openfish clob balance --asset-type collateral
```

## Configuration

### Wallet Setup

The CLI needs a private key to sign orders and on-chain transactions. It checks
credentials in this order:

1. CLI flag: `--private-key 0xabc...`
2. Environment variable: `OPENFISH_PRIVATE_KEY=0xabc...`
3. Config file: `~/.config/openfish/config.json`

```bash
# Create a new wallet and save it to local config
openfish wallet create

# Import an existing key
openfish wallet import 0xabc123...

# Show configured wallet metadata
openfish wallet show
```

Config file:

```json
{
  "private_key": "0x...",
  "chain_id": 56,
  "signature_type": "eoa"
}
```

### Signature Types

- `eoa` (default): signs directly with the configured key
- `proxy`: legacy proxy wallet flow
- `gnosis-safe`: signs for multisig wallet use

Override per command with `--signature-type eoa` or set
`OPENFISH_SIGNATURE_TYPE`.

### What Needs A Wallet

Most read-only commands do not need a wallet:

- market browsing
- event/tag/profile lookup
- order book, price, spread, and history queries
- public portfolio and market data

Wallet configuration is required for:

- placing and canceling CLOB orders
- checking authenticated balances, orders, trades, and account status
- CTF split/merge/redeem operations
- bridge deposits and withdrawals
- API key management

## Output Formats

Every command supports table output and JSON output:

```bash
# Human-readable output
openfish markets list --limit 2

# Machine-readable output
openfish -o json markets list --limit 2
```

Short form:

```bash
openfish -o json clob midpoint TOKEN_ID
openfish -o table clob midpoint TOKEN_ID
```

Errors follow the same pattern. Table mode prints `Error: ...`; JSON mode prints
`{"error":"..."}`. Failed commands return a non-zero exit code.

## Commands

### Markets

```bash
openfish markets list --limit 10
openfish markets list --active true --order volume_num
openfish markets list --closed false --limit 50 --offset 25
openfish markets get 12345
openfish markets get market-slug
openfish markets search "bitcoin" --limit 5
openfish markets tags 12345
```

Common `markets list` flags: `--limit`, `--offset`, `--order`,
`--ascending`, `--active`, `--closed`.

### Events And Metadata

```bash
openfish events list --limit 10
openfish events list --tag crypto --active true
openfish events get 500
openfish events tags 500

openfish tags list
openfish tags get crypto
openfish tags related crypto

openfish series list --limit 10
openfish comments list --entity-type event --entity-id 500
openfish profiles get 0xf5E6...
openfish sports list
```

### Order Book And Prices

Read-only CLOB commands do not need a wallet.

```bash
openfish clob ok
openfish clob price TOKEN_ID --side buy
openfish clob midpoint TOKEN_ID
openfish clob spread TOKEN_ID
openfish clob batch-prices "TOKEN1,TOKEN2" --side buy
openfish clob midpoints "TOKEN1,TOKEN2"
openfish clob spreads "TOKEN1,TOKEN2"
openfish clob book TOKEN_ID
openfish clob books "TOKEN1,TOKEN2"
openfish clob last-trade TOKEN_ID
openfish clob market 0xCONDITION_ID
openfish clob markets
openfish clob price-history TOKEN_ID --interval 1d --fidelity 30
openfish clob tick-size TOKEN_ID
openfish clob fee-rate TOKEN_ID
openfish clob neg-risk TOKEN_ID
openfish clob time
openfish clob geoblock
```

`price-history` interval options: `1m`, `1h`, `6h`, `1d`, `1w`, `max`.

### Trading

Trading commands require a configured wallet.

```bash
# Limit order
openfish clob create-order \
  --token TOKEN_ID \
  --side buy \
  --price 0.50 \
  --size 10

# Market order
openfish clob market-order \
  --token TOKEN_ID \
  --side buy \
  --amount 5

# Multiple orders
openfish clob post-orders \
  --tokens "TOKEN1,TOKEN2" \
  --side buy \
  --prices "0.40,0.60" \
  --sizes "10,10"

# Cancel orders
openfish clob cancel ORDER_ID
openfish clob cancel-orders "ORDER1,ORDER2"
openfish clob cancel-market --market 0xCONDITION_ID
openfish clob cancel-all

# View account trading state
openfish clob orders
openfish clob order ORDER_ID
openfish clob trades
openfish clob balance --asset-type collateral
openfish clob balance --asset-type conditional --token TOKEN_ID
openfish clob update-balance --asset-type collateral
```

Order types: `GTC`, `FOK`, `GTD`, `FAK`. Use `--post-only` for post-only
limit orders.

### Account, Rewards, And API Keys

```bash
openfish clob account-status
openfish clob notifications
openfish clob delete-notifications "NOTIF1,NOTIF2"

openfish clob rewards --date 2024-06-15
openfish clob earnings --date 2024-06-15
openfish clob earnings-markets --date 2024-06-15
openfish clob reward-percentages
openfish clob current-rewards
openfish clob market-reward 0xCONDITION_ID

openfish clob api-keys
openfish clob create-api-key
openfish clob delete-api-key
```

### On-Chain And Public Data

```bash
openfish data positions 0xWALLET_ADDRESS
openfish data closed-positions 0xWALLET_ADDRESS
openfish data value 0xWALLET_ADDRESS
openfish data traded 0xWALLET_ADDRESS
openfish data trades 0xWALLET_ADDRESS --limit 50
openfish data activity 0xWALLET_ADDRESS
openfish data holders 0xCONDITION_ID
openfish data open-interest 0xCONDITION_ID
openfish data volume 12345
openfish data leaderboard --period month --order-by pnl --limit 10
```

### Approvals

Before trading, Openfish contracts need ERC-20 and ERC-1155 approvals.

```bash
openfish approve check
openfish approve check 0xSOME_ADDRESS
openfish approve set
```

### CTF Operations

```bash
openfish ctf split --condition 0xCONDITION_ID --amount 10
openfish ctf merge --condition 0xCONDITION_ID --amount 10
openfish ctf redeem --condition 0xCONDITION_ID
openfish ctf redeem-neg-risk --condition 0xCONDITION_ID --amounts "10,5"

# ID helpers are read-only
openfish ctf condition-id --oracle 0xORACLE --question 0xQUESTION --outcomes 2
openfish ctf collection-id --condition 0xCONDITION_ID --index-set 1
openfish ctf position-id --collection 0xCOLLECTION_ID
```

`--amount` is in FISH units. On-chain CTF examples are separate from the
Openfish off-chain FISH balance deployment.

### Bridge

Bridge FISH between Openfish Balance and BSC. BNB is gas only unless you
explicitly quote and execute a BNB to FISH swap.

```bash
openfish bridge deposit 0xWALLET_ADDRESS
openfish bridge supported-assets
openfish bridge status 0xDEPOSIT_ADDRESS

openfish bridge withdraw 0xWALLET_ADDRESS \
  --to-chain-id 56 \
  --to-token-address 0xFISH_TOKEN \
  --recipient 0xRECIPIENT \
  --amount all

openfish bridge swap quote 0xWALLET_ADDRESS \
  --amount-in-wei 10000000000000000 \
  --to-token-address 0xFISH_TOKEN \
  --slippage-bps 100

openfish bridge swap execute 0xWALLET_ADDRESS --quote-id QUOTE_ID
openfish bridge swap status SWAP_ID
openfish bridge swap list 0xWALLET_ADDRESS --limit 20
```

### Wallet Management

```bash
openfish wallet create
openfish wallet create --force
openfish wallet import 0xKEY...
openfish wallet address
openfish wallet show
openfish wallet reset
openfish wallet reset --force
```

### Interactive Shell

```bash
openfish shell
# openfish> markets list --limit 3
# openfish> clob book TOKEN_ID
# openfish> exit
```

All shell commands work like CLI commands without the `openfish` prefix.

## Agent And Script Usage

Openfish CLI is designed to be called by scripts and OpenClaw-generated agents.
Use JSON output and pass credentials through runtime environment variables or a
local ignored config file.

```bash
openfish -o json markets list --limit 100
openfish -o json clob midpoint TOKEN_ID
openfish -o json clob orders
```

Secrets must not be pasted into chat, committed to generated agent workspaces,
or printed in logs. The CLI should load private keys, API secrets, and
passphrases from local runtime configuration only.

## Development

This repository contains two Rust crates:

- `openfish-cli`: the `openfish` command-line binary
- `openfish-client-sdk`: the Rust SDK used by the CLI

Run the public release checks:

```bash
python3 scripts/public_safety_scan.py
cargo fmt --all --check
cargo build --workspace --locked
cargo test --workspace --locked
```

Release instructions live in `RELEASE.md`.

## License

MIT
