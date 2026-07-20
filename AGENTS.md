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

## Syncing with the remote

"Sync with the remote" (or just "sync") is **bidirectional and always contacts
the remote** — it pulls *and* pushes. It is never push-only, and a clean local
working tree does **not** by itself mean "synced": a sync is not finished until
local and the remote have exchanged commits in both directions.

The steps for a sync:

1. `git fetch --all --prune` — see what the remote has.
2. `git pull` (which merges) — or `git merge` the upstream tracking branch —
   to integrate the remote's commits into your local branch **first**.
3. `git add` / `git commit` any local work.
4. `git push` — publish your commits.

Always integrate with **`git merge`** (and plain `git pull`, which merges).
**Do not `git rebase`** to sync — rebasing rewrites history and breaks shared
branches; keep the merge history instead.
