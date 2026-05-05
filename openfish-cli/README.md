# Openfish CLI

Rust CLI for Openfish. Browse markets, place orders, manage positions, and interact with onchain contracts — from a terminal or as a JSON API for scripts and agents.

> **Warning:** This is early, experimental software. Use at your own risk and do not use with large amounts of funds. APIs, commands, and behavior may change without notice. Always verify transactions before confirming.

## Install

### Build from source

```bash
git clone https://github.com/Openfish/openfish-cli
cd openfish-cli
cargo install --path openfish-cli
```

### Homebrew (macOS / Linux)

Available after the first public binary release:

```bash
brew tap Openfish/openfish-cli https://github.com/Openfish/openfish-cli
brew install openfish
```

### Shell script

Available after the first public binary release:

```bash
curl -sSL https://raw.githubusercontent.com/Openfish/openfish-cli/main/install.sh | sh
```

## Quick Start

```bash
# No wallet needed — browse markets immediately
openfish markets list --limit 5
openfish markets search "election"
openfish events list --tag politics

# Check a specific market
openfish markets get will-trump-win-the-2024-election

# JSON output for scripts
openfish -o json markets list --limit 3
```

To trade, set up a wallet:

```bash
openfish setup
# Or manually:
openfish wallet create
openfish approve set
```

## Configuration

### Wallet Setup

The CLI needs a private key to sign orders and on-chain transactions. Three ways to provide it (checked in this order):

1. **CLI flag**: `--private-key 0xabc...`
2. **Environment variable**: `OPENFISH_PRIVATE_KEY=0xabc...`
3. **Config file**: `~/.config/openfish/config.json`

```bash
# Create a new wallet (generates random key, saves to config)
openfish wallet create

# Import an existing key
openfish wallet import 0xabc123...

# Check what's configured
openfish wallet show
```

The config file (`~/.config/openfish/config.json`):

```json
{
  "private_key": "0x...",
  "chain_id": 137,
  "signature_type": "proxy"
}
```

### Signature Types

- `proxy` (default) — uses Openfish's proxy wallet system
- `eoa` — signs directly with your key
- `gnosis-safe` — for multisig wallets

Override per-command with `--signature-type eoa` or via `OPENFISH_SIGNATURE_TYPE`.

### What Needs a Wallet

Most commands work without a wallet — browsing markets, viewing order books, checking prices. You only need a wallet for:

- Placing and canceling orders (`clob create-order`, `clob market-order`, `clob cancel-*`)
- Checking your balances and trades (`clob balance`, `clob trades`, `clob orders`)
- On-chain operations (`approve set`, `ctf split/merge/redeem`)
- Reward and API key management (`clob rewards`, `clob create-api-key`)

## Output Formats

Every command supports `--output table` (default) and `--output json`.

```bash
# Human-readable table (default)
openfish markets list --limit 2
```

```
 Question                            Price (Yes)  Volume   Liquidity  Status
 Will Trump win the 2024 election?   52.00¢       $145.2M  $1.2M      Active
 Will BTC hit $100k by Dec 2024?     67.30¢       $89.4M   $430.5K    Active
```

```bash
# Machine-readable JSON
openfish -o json markets list --limit 2
```

```json
[
  { "id": "12345", "question": "Will Trump win the 2024 election?", "outcomePrices": ["0.52", "0.48"], ... },
  { "id": "67890", "question": "Will BTC hit $100k by Dec 2024?", ... }
]
```

Short form: `-o json` or `-o table`.

Errors follow the same pattern — table mode prints `Error: ...` to stderr, JSON mode prints `{"error": "..."}` to stdout. Non-zero exit code either way.

## Commands

### Markets

```bash
# List markets with filters
openfish markets list --limit 10
openfish markets list --active true --order volume_num
openfish markets list --closed false --limit 50 --offset 25

# Get a single market by ID or slug
openfish markets get 12345
openfish markets get will-trump-win

# Search
openfish markets search "bitcoin" --limit 5

# Get tags for a market
openfish markets tags 12345
```

**Flags for `markets list`**: `--limit`, `--offset`, `--order`, `--ascending`, `--active`, `--closed`

### Events

Events group related markets (e.g. "2024 Election" contains multiple yes/no markets).

```bash
openfish events list --limit 10
openfish events list --tag politics --active true
openfish events get 500
openfish events tags 500
```

**Flags for `events list`**: `--limit`, `--offset`, `--order`, `--ascending`, `--active`, `--closed`, `--tag`

### Tags, Series, Comments, Profiles, Sports

```bash
# Tags
openfish tags list
openfish tags get politics
openfish tags related politics
openfish tags related-tags politics

# Series (recurring events)
openfish series list --limit 10
openfish series get 42

# Comments on an entity
openfish comments list --entity-type event --entity-id 500
openfish comments get abc123
openfish comments by-user 0xf5E6...

# Public profiles
openfish profiles get 0xf5E6...

# Sports metadata
openfish sports list
openfish sports market-types
openfish sports teams --league NFL --limit 32
```

### Order Book & Prices (CLOB)

All read-only — no wallet needed.

```bash
# Check API health
openfish clob ok

# Prices
openfish clob price 48331043336612883... --side buy
openfish clob midpoint 48331043336612883...
openfish clob spread 48331043336612883...

# Batch queries (comma-separated token IDs)
openfish clob batch-prices "TOKEN1,TOKEN2" --side buy
openfish clob midpoints "TOKEN1,TOKEN2"
openfish clob spreads "TOKEN1,TOKEN2"

# Order book
openfish clob book 48331043336612883...
openfish clob books "TOKEN1,TOKEN2"

# Last trade
openfish clob last-trade 48331043336612883...

# Market info
openfish clob market 0xABC123...  # by condition ID
openfish clob markets             # list all

# Price history
openfish clob price-history 48331043336612883... --interval 1d --fidelity 30

# Metadata
openfish clob tick-size 48331043336612883...
openfish clob fee-rate 48331043336612883...
openfish clob neg-risk 48331043336612883...
openfish clob time
openfish clob geoblock
```

**Interval options for `price-history`**: `1m`, `1h`, `6h`, `1d`, `1w`, `max`

### Trading (CLOB, authenticated)

Requires a configured wallet.

```bash
# Place a limit order (buy 10 shares at $0.50)
openfish clob create-order \
  --token 48331043336612883... \
  --side buy --price 0.50 --size 10

# Place a market order (buy $5 worth)
openfish clob market-order \
  --token 48331043336612883... \
  --side buy --amount 5

# Post multiple orders at once
openfish clob post-orders \
  --tokens "TOKEN1,TOKEN2" \
  --side buy \
  --prices "0.40,0.60" \
  --sizes "10,10"

# Cancel
openfish clob cancel ORDER_ID
openfish clob cancel-orders "ORDER1,ORDER2"
openfish clob cancel-market --market 0xCONDITION...
openfish clob cancel-all

# View your orders and trades
openfish clob orders
openfish clob orders --market 0xCONDITION...
openfish clob order ORDER_ID
openfish clob trades

# Check balances
openfish clob balance --asset-type collateral
openfish clob balance --asset-type conditional --token 48331043336612883...
openfish clob update-balance --asset-type collateral
```

**Order types**: `GTC` (default), `FOK`, `GTD`, `FAK`. Add `--post-only` for limit orders.

### Rewards & API Keys (CLOB, authenticated)

```bash
openfish clob rewards --date 2024-06-15
openfish clob earnings --date 2024-06-15
openfish clob earnings-markets --date 2024-06-15
openfish clob reward-percentages
openfish clob current-rewards
openfish clob market-reward 0xCONDITION...

# Check if orders are scoring rewards
openfish clob order-scoring ORDER_ID
openfish clob orders-scoring "ORDER1,ORDER2"

# API key management
openfish clob api-keys
openfish clob create-api-key
openfish clob delete-api-key

# Account status
openfish clob account-status
openfish clob notifications
openfish clob delete-notifications "NOTIF1,NOTIF2"
```

### On-Chain Data

Public data — no wallet needed.

```bash
# Portfolio
openfish data positions 0xWALLET_ADDRESS
openfish data closed-positions 0xWALLET_ADDRESS
openfish data value 0xWALLET_ADDRESS
openfish data traded 0xWALLET_ADDRESS

# Trade history
openfish data trades 0xWALLET_ADDRESS --limit 50

# Activity
openfish data activity 0xWALLET_ADDRESS

# Market data
openfish data holders 0xCONDITION_ID
openfish data open-interest 0xCONDITION_ID
openfish data volume 12345  # event ID

# Leaderboards
openfish data leaderboard --period month --order-by pnl --limit 10
openfish data builder-leaderboard --period week
openfish data builder-volume --period month
```

### Contract Approvals

Before trading, Openfish contracts need ERC-20 (USDC) and ERC-1155 (CTF token) approvals.

```bash
# Check current approvals (read-only)
openfish approve check
openfish approve check 0xSOME_ADDRESS

# Approve all contracts (sends 6 on-chain transactions, needs POL for gas)
openfish approve set
```

### CTF Operations

Split, merge, and redeem conditional tokens directly on-chain.

```bash
# Split $10 USDC into YES/NO tokens
openfish ctf split --condition 0xCONDITION... --amount 10

# Merge tokens back to USDC
openfish ctf merge --condition 0xCONDITION... --amount 10

# Redeem winning tokens after resolution
openfish ctf redeem --condition 0xCONDITION...

# Redeem neg-risk positions
openfish ctf redeem-neg-risk --condition 0xCONDITION... --amounts "10,5"

# Calculate IDs (read-only, no wallet needed)
openfish ctf condition-id --oracle 0xORACLE... --question 0xQUESTION... --outcomes 2
openfish ctf collection-id --condition 0xCONDITION... --index-set 1
openfish ctf position-id --collection 0xCOLLECTION...
```

`--amount` is in FISH units. The `--partition` flag defaults to binary (`1,2`). On-chain CTF examples are not part of the Openfish off-chain FISH deployment.

### Bridge

Bridge FISH between Openfish Balance and BSC. BNB is gas only unless you explicitly quote and execute a BNB -> FISH swap.

```bash
# Get the BSC deposit address for an Openfish wallet
openfish bridge deposit 0xWALLET_ADDRESS

# Withdraw FISH from Openfish Balance to a BSC address
openfish bridge withdraw 0xWALLET_ADDRESS --to-chain-id 56 --to-token-address 0xFISH_TOKEN --recipient 0xRECIPIENT --amount all

# List supported FISH bridge asset
openfish bridge supported-assets

# Check deposit / withdrawal status
openfish bridge status 0xDEPOSIT_ADDRESS

# Quote BNB -> FISH without sending a transaction
openfish bridge swap quote 0xWALLET_ADDRESS --amount-in-wei 10000000000000000 --to-token-address 0xFISH_TOKEN --slippage-bps 100

# Execute a quoted BNB -> FISH swap
openfish bridge swap execute 0xWALLET_ADDRESS --quote-id <QUOTE_ID>

# Check swap status
openfish bridge swap status <SWAP_ID>

# List swap history for a wallet
openfish bridge swap list 0xWALLET_ADDRESS --limit 20
```

### Wallet Management

```bash
openfish wallet create               # Generate new random wallet
openfish wallet create --force       # Overwrite existing
openfish wallet import 0xKEY...      # Import existing key
openfish wallet address              # Print wallet address
openfish wallet show                 # Full wallet info (address, source, config path)
openfish wallet reset                # Delete config (prompts for confirmation)
openfish wallet reset --force        # Delete without confirmation
```

### Interactive Shell

```bash
openfish shell
# openfish> markets list --limit 3
# openfish> clob book 48331043336612883...
# openfish> exit
```

Supports command history. All commands work the same as the CLI, just without the `openfish` prefix.

### Other

```bash
openfish status     # API health check
openfish setup      # Guided first-time setup wizard
openfish upgrade    # Update to the latest version
openfish --version
openfish --help
```

## Common Workflows

### Browse and research markets

```bash
openfish markets search "bitcoin" --limit 5
openfish markets get bitcoin-above-100k
openfish clob book 48331043336612883...
openfish clob price-history 48331043336612883... --interval 1d
```

### Set up a new wallet and start trading

```bash
openfish wallet create
openfish approve set                    # needs POL for gas
openfish clob balance --asset-type collateral
openfish clob market-order --token TOKEN_ID --side buy --amount 5
```

### Monitor your portfolio

```bash
openfish data positions 0xYOUR_ADDRESS
openfish data value 0xYOUR_ADDRESS
openfish clob orders
openfish clob trades
```

### Place and manage limit orders

```bash
# Place order
openfish clob create-order --token TOKEN_ID --side buy --price 0.45 --size 20

# Check it
openfish clob orders

# Cancel if needed
openfish clob cancel ORDER_ID

# Or cancel everything
openfish clob cancel-all
```

### Script with JSON output

```bash
# Pipe market data to jq
openfish -o json markets list --limit 100 | jq '.[].question'

# Check prices programmatically
openfish -o json clob midpoint TOKEN_ID | jq '.mid'

# Error handling in scripts
if ! result=$(openfish -o json clob balance --asset-type collateral 2>/dev/null); then
  echo "Failed to fetch balance"
fi
```

## Architecture

```
src/
  main.rs        -- CLI entry point, clap parsing, error handling
  auth.rs        -- Wallet resolution, RPC provider, CLOB authentication
  config.rs      -- Config file (~/.config/openfish/config.json)
  shell.rs       -- Interactive REPL
  commands/      -- One module per command group
  output/        -- Table and JSON rendering per command group
```

## License

MIT
