# 🦀 Solana Infrastructure Scaffold (Rust)

> **Status: Active Development**  
> **Goal:** A production-ready, modular boilerplate for building high-performance Solana indexers, bots, and tools.  
> **Origin:** Built as a direct response to the [On-Chain Event Indexer Post-Mortem], implementing Hexagonal Architecture and strict Type-Driven Design to solve scalability and maintenance bottlenecks.

## Strategic Objective

To provide a rock-solid foundation for Web3 infrastructure that handles asynchronous state, complex RPC orchestration, and resilient data persistence without the "monolith trap."

---

## Planned Architecture (Hexagonal)

The project is strictly divided into three layers to ensure that business logic remains independent of external volatile APIs (like Helius or QuickNode).

### 1. Core Domain

- **Entities:** Pure Rust structures using the **New Type Pattern** (e.g., `Lamports(u64)`, `SolanaAddress(String)`).
- **Logic:** Stateless business rules and data transformation logic.
- **Ports (Traits):** Definitions of what the system _needs_ (e.g., `trait SolanaProvider`, `trait Repository`).

### 2. Infrastructure

- **Adapters:** Concrete implementations of Ports.
  - `HeliusClient`: JSON-RPC orchestration with rate-limiting.
  - `PostgresRepo`: SQLx-backed persistence with automated migrations.
- **External Services:** Configuration for Docker, Redis (if needed), and Prometheus.

### 3. Application Layer

- **Service Orchestration:** Coordinating between Ports and Domain logic.
- **Entry Points:**
  - `Clap-based CLI`: For manual control and daemon management.
  - `Axum-based API`: For headless integration and health monitoring.

---

## 🛠 Tech Stack & Engineering Standards

- **Runtime:** `Tokio` (Multi-threaded, async-first).
- **API Layer:** `Axum` (Type-safe routing and state management).
- **Database:** `PostgreSQL` with `sqlx` (Compile-time verified queries and migrations).
- **Configuration:** Layered config using `config-rs` (YAML/JSON + Environment overrides).
- **Type Safety:** Heavy use of the **New Type Pattern** to prevent primitive obsession (e.g., `SolanaAddress` instead of `String`).
- **Observability:** Structured logging with `tracing` and `OpenTelemetry` integration.
- **Error Handling:** Domain-specific error enums using `thiserror`.
