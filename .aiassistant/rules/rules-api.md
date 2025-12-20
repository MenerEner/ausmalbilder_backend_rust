---
apply: always
---

## API Design Rules (REST, Richardson Maturity Model Level 2)

### 1) Target level
- The API must follow **Richardson Maturity Model Level 2**:
    - ✅ Resource-oriented URIs
    - ✅ Proper use of HTTP verbs
    - ✅ Proper use of HTTP status codes
    - ✅ Content negotiation (at minimum `application/json`)
    - ❌ No requirement for HATEOAS links (Level 3)

### 2) Resource naming & URL conventions
- Use **nouns**, not verbs, in paths.
- Prefer **plural** resource collections:
    - ✅ `/orders`, `/users`, `/prices`
- Nested resources only when it represents a clear containment relationship:
    - ✅ `/orders/{order_id}/items`
    - ❌ `/users/{user_id}/create-order`
- Use lowercase, hyphen-separated segments when needed:
    - ✅ `/price-quotes` (if it’s a resource)

### 3) HTTP method semantics
- `GET`:
    - Must be safe/idempotent
    - Used for reads and list endpoints
- `POST`:
    - Create a new resource in a collection (`POST /orders`)
    - Or trigger a server-side process that results in a created resource (`POST /price-quotes`)
- `PUT`:
    - Replace a full resource representation (rare unless you truly support full replace)
- `PATCH`:
    - Partial updates (preferred for updates)
- `DELETE`:
    - Delete a resource (or soft-delete if your domain requires it)

### 4) Status codes (minimum standard)
- `200 OK`: successful read/update returning a body
- `201 Created`: successful creation
    - Must include `Location: /resource/{id}` header when applicable
- `204 No Content`: successful delete or update with no body
- `400 Bad Request`: malformed request (validation at API boundary)
- `401 Unauthorized`: missing/invalid auth
- `403 Forbidden`: authenticated but not allowed
- `404 Not Found`: resource not found
- `409 Conflict`: business conflict (e.g., version/state conflict)
- `422 Unprocessable Entity`: semantic validation failure (optional; use consistently if used)
- `429 Too Many Requests`: rate limit/backpressure
- `500/502/503`: server/upstream failures (don’t leak internals)

### 5) Request/response body conventions
- Use JSON with explicit DTOs (do not expose domain models).
- Follow a consistent envelope for errors (example shape; keep stable):
    - `{ "error": { "code": "ORDER_ALREADY_CONFIRMED", "message": "...", "correlation_id": "..." } }`
- Avoid ambiguous fields; use explicit names and stable types.

### 6) Pagination, filtering, sorting (collections)
- All collection endpoints must support pagination.
- Standard query parameters:
    - `page` + `page_size` (or `limit` + `cursor`, pick one and stick to it)
    - Optional `sort` (e.g. `sort=created_at,-status`)
    - Optional filters via query params (e.g. `status=confirmed`)
- Enforce max page size to protect the service.

### 7) Idempotency & safe retries
- For operations where clients may retry (`POST` that creates resources), support an **Idempotency-Key** header when needed.
- If not implementing now, the assistant must at least:
    - design endpoints so retries are safe where possible
    - avoid side effects on `GET`

### 8) Versioning & compatibility
- Prefer **non-breaking** changes:
    - add fields (backward compatible)
    - avoid removing/renaming fields
- If versioning is required, prefer URL prefix:
    - `/v1/orders`
- Versioning strategy must be consistent across the API.

### 9) Observability & correlation
- Every request should have a correlation/request id:
    - accept incoming `x-correlation-id` (or generate one)
    - include it in logs and error responses
- Use `tracing` spans per request with method/path/status/latency.

### 10) Axum implementation guidelines
- Use typed extractors (`Path`, `Query`, `Json`, `State`).
- Map application errors to HTTP via a single `AppError: IntoResponse`.
- Keep handlers thin: DTO ↔ command mapping, call use-case, map result ↔ response.

### 11) Naming: commands vs resources
- Avoid “RPC verbs” in URLs:
    - ❌ `/orders/{id}/confirm`
- Prefer state transitions via sub-resource or PATCH:
    - ✅ `PATCH /orders/{id}` with `{ "status": "confirmed" }`
    - or ✅ `POST /orders/{id}/confirmations` if you model confirmations as a resource
- When in doubt, model “actions” as resources (Level 2-friendly):
    - e.g. `POST /price-quotes` creates a quote resource
