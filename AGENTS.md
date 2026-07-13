# ⚠️ DEPRECATED — this repository is outmoded

**`fiducia-memory` is no longer the source of truth. Do not develop here.**

This service was reconciled into the canonical, Rust-only repository:

> ## → **[`fiducia-cloud/fiducia-memory.rs`](https://github.com/fiducia-cloud/fiducia-memory.rs)**

All code from this repo was merged there (its full history is preserved as a
merge parent, so nothing was lost). The naming convention is that strictly-Rust
services carry the `.rs` suffix; this non-suffixed repo is the older duplicate.

## For agents (human or AI)

- **Do NOT** create branches, commits, PRs, or worktrees here.
- **Do NOT** treat this code as current — it is superseded.
- Send all new work to `fiducia-memory.rs`. The original wire/API types from this repo are
  preserved there (in `fiducia-memory.rs` behind a `compat-service` feature / `src/durable/`).
- This repository is retained read-only for history and should be archived.

---

## Prior policy (retained for reference)


- Work directly on main.
- Do not create feature branches or worktrees.
- Preserve uncommitted work.
- Push completed work to origin/main.
