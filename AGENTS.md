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

"Sync with the remote" (or just "sync") is a **two-way** exchange — pull the
remote's commits down **and** push yours up. It is never push-only, and a clean
local tree does not by itself mean "synced": you are done only once local and
the remote hold the same commits.

To sync:

1. **Commit your work first** (`git add` + `git commit`) so the tree is clean —
   pull/merge only into a clean tree. `git pull` / `git merge` aborts when an
   incoming change touches a file you have edited, and even when it doesn't it
   buries the merge in your uncommitted work. (Can't commit yet? `git stash`,
   then `git stash pop` after step 3.)
2. `git fetch --all --prune` — safe any time; it only updates tracking refs.
3. `git pull` (fetch + merge) — or `git merge` the upstream branch — to
   integrate the remote's commits.
4. `git push` to publish yours.

Integrate with **`git merge` / `git pull`**. **Never `git rebase` to sync** — it
rewrites history and breaks shared branches.
