---
apply: always
---

## Git Usage Rules

### 1) Safety first
- The agent must **never create commits** unless the user **explicitly asks** to commit.
- The agent must **never push** to any remote unless the user **explicitly asks** to push.
- The agent must **never** rewrite history (`rebase`, `reset --hard`, `push --force`) unless explicitly requested **and** the risk is explained.

### 2) Default workflow expectations
- Prefer **small, focused changes** that are easy to review.
- Keep refactors separate from behavior changes when possible.
- Always provide a **clean diff**:
    - suggest `git diff` / `git status` checks before any risky step
    - avoid unrelated file churn (formatting, renames) unless requested

### 3) Branching & naming (recommended)
- Use short-lived feature branches:
    - `feature/<topic>`
    - `fix/<issue>`
    - `chore/<task>`
- Keep branch names lowercase and hyphenated.

### 4) Commit message standards (only when asked to commit)
If the user explicitly requests a commit:
- Use imperative, concise messages:
    - `feat: add order confirmation endpoint`
    - `fix: handle empty config path`
    - `chore: update workspace dependencies`
- Include scope when helpful:
    - `feat(api): add pagination to orders`
    - `fix(infra): wrap SeaORM errors into AppError`
- One logical change per commit.

### 5) Pre-commit quality gates
Before any commit (when requested), the agent should ensure:
- `cargo fmt`
- `cargo clippy` (prefer with warnings denied)
- `cargo test`

If any step fails, the agent must report:
- what failed
- why it likely failed
- the minimal fix

### 6) Handling generated or large changes
- Do not commit generated code (e.g., SeaORM entities) unless explicitly requested.
- Avoid committing large dependency lockfile changes unless required and explained.
- If migrations are involved, commit migrations together with the code that needs them.

### 7) Merge strategy (only when asked)
- Prefer PR-style merges over direct `main` commits.
- Avoid squash/merge/rebase decisions unless the user specifies the repo’s policy.

---