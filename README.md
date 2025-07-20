# Maestro Esplora Proxy

Authenticated Esplora proxy for BDK applications.

```
BDK Client → Proxy (localhost:8080) → Maestro Esplora API
```

### Requirements

-   [Maestro API key](https://dashboard.gomaestro.org)

### Configuration

Set the appropriate Maestro Esplora API endpoint based on your network.

**Mainnet:**

```rust
static ESPLORA_URL: &str = "https://xbt-mainnet.gomaestro-api.org/v0/esplora";
```

**Testnet (testnet4):**

```rust
static ESPLORA_URL: &str = "https://xbt-testnet.gomaestro-api.org/v0/esplora";
```

Set your API key

**API Key:**

```rust
static API_KEY: &str = "your_actual_api_key_here";
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
