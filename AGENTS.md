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
the remote** — it fetches *and* pushes, never push-only. A clean local working
tree does **not** by itself mean "synced": a sync is not finished until local
and the remote have exchanged commits in both directions.

How to sync:

1. `git fetch --all --prune` — always safe; it only updates remote-tracking
   refs and never touches your working tree, so run it any time.
2. Make the working tree **clean before you pull/merge**: `git add` +
   `git commit` your work (or `git stash`). **Only `git pull` / `git merge`
   when the tree is not dirty** — pulling into a dirty tree makes git refuse
   the merge or tangle uncommitted edits with the incoming commits.
3. `git pull` (which fetches + merges) — or `git merge` the upstream tracking
   branch — to integrate the remote's commits into your now-clean branch.
4. `git push` — publish your commits so the remote has them too.

Integrate with **`git merge`** / **`git pull`** (which merges). **Never
`git rebase`** to sync — it rewrites history and breaks shared branches.
