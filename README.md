# Maestro Esplora Proxy

Authenticated Esplora proxy for BDK applications.

```
BDK Client → Proxy (localhost:8080) → Maestro Esplora API
```

### Requirements

-   [Maestro API key](https://dashboard.gomaestro.org)

### Configuration

Clone Maestro-Esplora-Proxy Repo:

```bash
git clone https://github.com/maestro-org/maestro-esplora-proxy.git && cd maestro-esplora-proxy
```

Copy `.env.example` template:

```bash
cp .env.example .env
```

Edit your `.env` file:

```bash
MAESTRO_API_KEY=your-maestro-api-key-here

# mainnet
ESPLORA_URL=https://xbt-mainnet.gomaestro-api.org/v0/esplora

# testnet4
# ESPLORA_URL=https://xbt-testnet.gomaestro-api.org/v0/esplora
```

### Usage

**Run proxy:**

```bash
cargo run
```

Within your client-side code, connect to the _locally running_ Esplora Proxy.

_Example_

```rust
let blockchain = EsploraBlockchain::new("http://localhost:8080", 20);
```

### Support

-   [Open an issue](https://github.com/maestro-org/maestro-esplora-proxy/issues/new)
-   [Join Discord](https://discord.gg/ES2rDhBJt3)
