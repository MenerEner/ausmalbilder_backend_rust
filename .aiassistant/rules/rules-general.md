---
apply: always
---

# Rust AI Coding Assistant Instructions (DDD + Axum/Tokio/Tracing/Serde)

## 0) Project assumptions
- This is a Rust **workspace** following **Domain-Driven Design (DDD)**.
- Each domain lives in its **own crate** (`domain-*`).
- The runtime/API layer is an **Axum (0.8)** HTTP service on **Tokio**.
- Observability uses **`tracing`**.
- Serialization uses **`serde`** (JSON likely at the API edge).
- There is a **`shared`** crate currently responsible for **config loading** (yaml via `config`).

---

## 1) Architecture: hard rules (must obey)

### 1.1 Crate boundaries
- **Domain crates must be pure**:
    - ❌ No Axum/Tokio types in `domain-*`
    - ❌ No HTTP, DB, filesystem, network, config parsing in `domain-*`
    - ✅ Only business logic, invariants, domain types, domain behavior
- **Dependencies flow inward only**:
    - `api` → `application` → `domain-*`
    - `api` → `infrastructure` → `domain-*` (only if infrastructure needs domain types)
    - ❌ `domain-*` must never depend on `api`, `infrastructure`, or `application`
- **Edge concerns stay at the edge (default)**:
    - Serde derives and HTTP types belong in `api` (DTOs), not in domain models by default.
    - Logging belongs in `api` / `application` by default.

### 1.2 Keep `shared` small
- `shared` must remain a **tiny kernel** (currently: config types + config loader).
- ❌ Do not turn `shared` into a “misc utils dumping ground”.
- Prefer returning errors from `shared`; log them at the boundary (`api`/`application`).

---

## 2) Recommended workspace layout

```text
crates/
  shared/                # config structs + loader (yaml)
  domain-orders/         # pure domain
  domain-pricing/
  domain-users/
  application/           # use-cases + ports (traits)
  infrastructure/        # adapters: repo implementations, external clients
  api/                   # axum router, handlers, DTOs, middleware
```


## Responsibilities
- **`domain-*`**: invariants, aggregates, entities, value objects, domain services (pure).
- **`application`**: orchestration/use-cases; owns ports/traits; calls domain behavior.
- **`infrastructure`**: implements ports (repositories/clients); integration code.
- **`api`**: HTTP boundary; DTO mapping; error-to-response mapping; middleware/tracing.
- **`shared`**: config-only (typed Settings + loader).

---

## 3) Dependency policy (what to import where)

### 3.1 Domain crates (`domain-*`)
- Keep dependencies minimal.
- Default: **no `serde`, no `tokio`, no `axum`, no `config`.**
- Domain types should enforce invariants via constructors/factories.
- Prefer newtypes and value objects (`OrderId`, `Money`, `Email`) over raw primitives.

#### Optional: serde in domains (feature-gated)
If serialization is truly needed for a domain type, gate it behind a feature:
- Use `cfg_attr(feature = "serde", derive(Serialize, Deserialize))`
- Keep it opt-in and not required for core domain compilation.

### 3.2 Application crate
- Owns **use-cases** and **ports/traits** (e.g., repositories).
- May use `tracing` for use-case spans.
- ❌ No Axum types. No HTTP response building here.

### 3.3 Infrastructure crate
- Implements application ports (repositories, external services).
- Owns integration concerns: retries, timeouts, mapping, persistence details.

### 3.4 API crate (Axum)
- Axum router, handlers, extractors, middleware.
- DTOs use serde; domain models generally do not.
- All HTTP error mapping happens here.

### 3.5 Shared crate
- `Settings` struct(s) + `load_settings()` function.
- Return errors, don’t log unless explicitly requested.
- Keep `tracing` out of `shared` unless required (prefer logging at startup in `api`).

---

## 4) Decision: Domain tracing is allowed via Option A (feature-gated)

### 4.1 Rule
- Domain crates **may use `tracing` internally only** when the crate feature `tracing` is enabled.
- When the feature is disabled, domain tracing must be **zero-overhead** (calls compile to no-ops).

### 4.2 Implementation requirements (what the assistant must do)
- In each `domain-*` crate:
  - Add an optional dependency: `tracing = { version = "0.1", optional = true }`
  - Add a feature: `[features] tracing = ["dep:tracing"]`
- Avoid sprinkling `#[cfg(feature = "tracing")]` everywhere:
  - Define **domain-local no-op macros** (e.g., `ddebug!`, `dinfo!`, `dwarn!`, `derror!`) in a single module (e.g., `telemetry.rs`), and use those macros in domain code.
- If instrumenting functions:
  - Use `cfg_attr(feature = "tracing", tracing::instrument(...))` so the attribute disappears when the feature is off.
- Do **not** leak `tracing` types into domain public APIs:
  - ❌ Don’t return/accept `Span`, `Subscriber`, etc.
- Do not log secrets/PII from domains (IDs only; redact where needed).

### 4.3 Feature aggregation for ergonomics
- The assistant should prefer a single top-level feature toggle:
  - `application` defines a feature like `domain-tracing` that enables `domain-*/tracing` features
  - `api` re-exports `domain-tracing` from `application`
- Result:
  - `cargo run -p api --features domain-tracing` enables domain-level tracing across all domains.

---

## 5) Axum handler rules (API layer)
- Handlers must be **small and boring**:
  - Extract → call application/service → map result → response
- Prefer typed extractors:
  - `State`, `Path`, `Query`, `Json`, headers, etc.
- ❌ No business logic in handlers. Put business logic in `application` / domain.
- State:
  - Use `State<AppState>`
  - Store shared resources behind `Arc` if needed
  - Avoid global mutables

---

## 6) Error handling rules (service-grade)

### 6.1 No casual panics
- ❌ No `unwrap()` / `expect()` in request paths or library-like code.
- Use `Result` + `?`.
- Only allow `unwrap()` in tests or in clearly documented “impossible” invariants.

### 6.2 One app error type at the boundary
- Use an `AppError` at the application/API boundary.
- In `api`, implement `IntoResponse` for `AppError`:
  - map to appropriate HTTP status code
  - return a stable JSON error shape
- Don’t leak internals to clients. Log details server-side.

---

## 7) Tracing rules (structured, actionable)
- Use `tracing` macros (`info!`, `warn!`, `error!`, `debug!`) with **structured fields**:
  - Example: `info!(user_id = %user_id, order_id = %order_id, "created order")`
- Prefer `#[tracing::instrument]` on **application/service functions** (not necessarily every handler):
  - Use `skip(state, large_payload)` to avoid logging large/sensitive data
- Don’t log secrets/PII. Redact at boundaries.
- Avoid log spam in loops; log summaries.

---

## 8) Tokio/async rules
- ❌ No blocking calls on async tasks.
  - If blocking is unavoidable: use `spawn_blocking` or a dedicated threadpool.
- Always consider:
  - cancellation (client disconnect/shutdown)
  - timeouts for external calls
  - backpressure (bounded channels if queues are used)
- Spawning:
  - Avoid “fire-and-forget” unless explicitly safe and desired
  - Prefer keeping `JoinHandle`s when correctness matters

---

## 9) Serde/API stability rules
- Use **DTO structs** at the HTTP boundary (`api`), not domain models.
- Request/response DTOs:
  - `#[derive(Serialize, Deserialize)]`
  - explicit field naming; use `rename` only when required
- Prefer strictness for inbound DTOs where helpful (avoid accepting garbage silently).
- Never expose internal domain structure by accident; keep domain and API representations separate.

---

## 10) Ports & adapters (DDD in Rust)
- Define repository/client **traits (ports)** in the `application` crate.
- Implement those traits in `infrastructure`.
- Domain remains persistence-ignorant.

---

## 11) Testing expectations
- Every new feature must include:
  - Unit tests for domain and application logic
  - At least one test covering endpoint wiring/mapping when adding routes
- Tests must be deterministic; avoid sleeps unless unavoidable.

---

## 12) Dependency discipline
- Do not introduce new crates casually.
- Add a new dependency only if it:
  - materially reduces risk (security/correctness), or
  - avoids re-implementing complex/well-known functionality, or
  - is explicitly requested
- If proposing a crate, explain:
  - why it’s needed
  - what it replaces
  - any tradeoffs (size, maintenance, features)

---

## 13) Output requirements for the assistant
When providing code changes:
- Prefer **minimal diffs** and incremental edits.
- Keep refactors separate from behavior changes whenever possible.
- Provide the commands to run:
  - `cargo fmt`
  - `cargo clippy` (ideally with warnings denied)
  - `cargo test`
- Ensure code is rustfmt/clippy clean; do not introduce new warnings.

---

### 10.1 Database rules (SeaORM)

- **SeaORM is infrastructure-only**:
  - ✅ Allowed in `infrastructure` (and optionally a dedicated `migration` crate)
  - ❌ Not allowed in `domain-*`, `application`, or `shared`

- **No persistence leakage across boundaries**:
  - `application` defines repository **traits (ports)**.
  - `infrastructure` implements those ports using SeaORM.
  - ❌ Do not expose SeaORM types (`Entity/Model/ActiveModel`, `DatabaseConnection`, `DbErr`, query builders) in public APIs of `application` or `domain-*`.

- **Entities vs domain models**:
  - SeaORM `Entity/Model/ActiveModel` represent the **database schema**, not the domain.
  - Mapping between persistence models and domain types happens **inside infrastructure** (e.g., `impl TryFrom<DbModel> for DomainType` in `infrastructure`).

- **Module layout suggestion (inside `infrastructure`)**:
  - `infrastructure::db::entities` (SeaORM generated entities)
  - `infrastructure::db::repos` (repo implementations using SeaORM)
  - `infrastructure::db::mapper` (domain ↔ db mapping helpers)

- **Connection lifecycle**:
  - Create **one** `DatabaseConnection` at startup (in the binary, e.g., `api/main.rs`).
  - Store it in `AppState` (or an infra container) and pass it into repositories/services.
  - Do not create new connections per request.

- **Config sourcing**:
  - DB URL / pool config must come from typed config in `shared` (loaded by the binary).
  - ❌ No reading env/config files inside `infrastructure` modules.

- **Transactions**:
  - For multi-step use-cases requiring atomicity, run them inside a transaction.
  - Prefer: `application` calls a port like `TransactionManager` / `UnitOfWork` (implemented in `infrastructure` with SeaORM transactions).
  - If you keep it simpler initially: repositories may accept `&DatabaseConnection`, and infrastructure can internally use SeaORM transactions where needed—without exposing `DatabaseTransaction` outside `infrastructure`.

- **Migrations**:
  - Prefer a dedicated crate (e.g., `migration`) using `sea-orm-migration`.
  - Run migrations via CI/CD or an explicit command.
  - ❌ Do not auto-run migrations on app startup in production.

- **Query hygiene**:
  - Avoid N+1 queries: use relations (`find_related`) or explicit joins.
  - Enforce pagination on list endpoints and set sensible defaults/limits.

- **Observability**:
  - DB errors are logged at `infrastructure` / `application` boundaries with `tracing`.
  - Never log secrets/PII or raw connection strings.

---