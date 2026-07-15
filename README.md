# fiducia-memory

> **⚠️ DEPRECATED.** This repository is outmoded and superseded by
> [`fiducia-memory.rs`](https://github.com/fiducia-cloud/fiducia-memory.rs),
> which carries the current durable-memory service (and is the monorepo
> submodule). Do not develop here; the description below is retained for
> historical reference. See `AGENTS.md` for the deprecation policy.

Tenant-scoped durable memory for Fiducia agents. It stores immutable provenance-bearing claims in PostgreSQL/pgvector, tracks supersession without destroying history, and combines full-text and cosine similarity for hybrid recall.

Set DATABASE_URL to PostgreSQL with pgvector available, then run cargo run. The service migrates its schema on startup and binds to 127.0.0.1:8090 by default. FIDUCIA_MEMORY_BIND overrides it.

- POST /v1/claims appends a claim and its 1536-dimensional embedding.
- POST /v1/claims/{claim_id}/supersede atomically closes an active claim and appends its replacement.
- POST /v1/recall performs tenant-filtered hybrid semantic and lexical recall.
- GET /healthz is process liveness; GET /readyz verifies PostgreSQL.

The caller creates embeddings. Every request carries a tenant_id; production ingress must authenticate the caller and bind that tenant to its credential.
