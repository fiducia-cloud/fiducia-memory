use crate::model::{AppendClaim, Claim, RecallHit, RecallRequest};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, QueryResult,
    Statement, TransactionTrait,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryStore {
    database: DatabaseConnection,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("claim not found")]
    NotFound,
    #[error("claim belongs to another tenant")]
    TenantMismatch,
    #[error(transparent)]
    Database(#[from] DbErr),
}

impl MemoryStore {
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn migrate(&self) -> Result<(), DbErr> {
        self.database
            .execute_unprepared(include_str!("../migrations/0001_memory.sql"))
            .await?;
        Ok(())
    }

    pub async fn ping(&self) -> Result<(), DbErr> {
        self.database
            .query_one(Statement::from_string(
                DbBackend::Postgres,
                "SELECT 1 AS ready",
            ))
            .await?;
        Ok(())
    }

    pub async fn append(
        &self,
        input: &AppendClaim,
        embedding: Vec<f32>,
    ) -> Result<Claim, StoreError> {
        let tx = self.database.begin().await?;
        let claim = insert_claim(&tx, input, &embedding).await?;
        tx.commit().await?;
        Ok(claim)
    }

    pub async fn supersede(
        &self,
        old_id: Uuid,
        tenant_id: Uuid,
        input: &AppendClaim,
        embedding: Vec<f32>,
    ) -> Result<Claim, StoreError> {
        let tx = self.database.begin().await?;
        let updated = tx
            .execute(Statement::from_sql_and_values(
                DbBackend::Postgres,
                "UPDATE memory_claims SET valid_until = COALESCE(valid_until, now()) WHERE claim_id = $1 AND tenant_id = $2 AND valid_until IS NULL",
                [old_id.into(), tenant_id.into()],
            ))
            .await?;
        if updated.rows_affected() == 0 {
            return Err(StoreError::NotFound);
        }
        let mut replacement = input.clone_for_supersede(old_id, tenant_id);
        let claim = insert_claim(&tx, &replacement, &embedding).await?;
        replacement.content.clear();
        tx.commit().await?;
        Ok(claim)
    }

    pub async fn recall(
        &self,
        request: &RecallRequest,
        embedding: Vec<f32>,
    ) -> Result<Vec<RecallHit>, StoreError> {
        let lexical_weight = 1.0 - request.semantic_weight;
        let rows = self
            .database
            .query_all(Statement::from_sql_and_values(
                DbBackend::Postgres,
                include_str!("../sql/recall.sql"),
                [
                    request.tenant_id.into(),
                    request.query.clone().into(),
                    vector_literal(&embedding).into(),
                    request.semantic_weight.into(),
                    lexical_weight.into(),
                    request.limit.into(),
                ],
            ))
            .await?;
        rows.iter()
            .map(recall_hit_from_row)
            .collect::<Result<Vec<_>, _>>()
            .map_err(StoreError::from)
    }
}

async fn insert_claim(
    tx: &DatabaseTransaction,
    input: &AppendClaim,
    embedding: &[f32],
) -> Result<Claim, DbErr> {
    let digest = format!("{:x}", Sha256::digest(input.content.as_bytes()));
    let row = tx
        .query_one(Statement::from_sql_and_values(
            DbBackend::Postgres,
            "INSERT INTO memory_claims (tenant_id, subject, predicate, object, source, confidence, content, content_sha256, embedding, valid_from, valid_until, supersedes_claim_id) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9::vector,COALESCE($10,now()),$11,$12) RETURNING claim_id,tenant_id,subject,predicate,object,source,confidence,content,content_sha256,valid_from,valid_until,supersedes_claim_id,created_at",
            [
                input.tenant_id.into(),
                input.subject.trim().into(),
                input.predicate.trim().into(),
                input.object.clone().into(),
                input.source.clone().into(),
                input.confidence.into(),
                input.content.trim().into(),
                digest.into(),
                vector_literal(embedding).into(),
                input.valid_from.into(),
                input.valid_until.into(),
                input.supersedes_claim_id.into(),
            ],
        ))
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("inserted memory claim".to_string()))?;
    claim_from_row(&row)
}

fn recall_hit_from_row(row: &QueryResult) -> Result<RecallHit, DbErr> {
    Ok(RecallHit {
        claim: claim_from_row(row)?,
        lexical_score: row.try_get("", "lexical_score")?,
        semantic_score: row.try_get("", "semantic_score")?,
        score: row.try_get("", "score")?,
    })
}

fn claim_from_row(row: &QueryResult) -> Result<Claim, DbErr> {
    Ok(Claim {
        claim_id: row.try_get("", "claim_id")?,
        tenant_id: row.try_get("", "tenant_id")?,
        subject: row.try_get("", "subject")?,
        predicate: row.try_get("", "predicate")?,
        object: row.try_get("", "object")?,
        source: row.try_get("", "source")?,
        confidence: row.try_get("", "confidence")?,
        content: row.try_get("", "content")?,
        content_sha256: row.try_get("", "content_sha256")?,
        valid_from: row.try_get("", "valid_from")?,
        valid_until: row.try_get("", "valid_until")?,
        supersedes_claim_id: row.try_get("", "supersedes_claim_id")?,
        created_at: row.try_get("", "created_at")?,
    })
}

fn vector_literal(values: &[f32]) -> String {
    use std::fmt::Write as _;

    let mut literal = String::with_capacity(values.len() * 8);
    literal.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            literal.push(',');
        }
        write!(&mut literal, "{value}").expect("writing to String cannot fail");
    }
    literal.push(']');
    literal
}

impl AppendClaim {
    fn clone_for_supersede(&self, old_id: Uuid, tenant_id: Uuid) -> Self {
        Self {
            tenant_id,
            subject: self.subject.clone(),
            predicate: self.predicate.clone(),
            object: self.object.clone(),
            source: self.source.clone(),
            confidence: self.confidence,
            content: self.content.clone(),
            embedding: self.embedding.clone(),
            valid_from: self.valid_from,
            valid_until: self.valid_until,
            supersedes_claim_id: Some(old_id),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn vector_literal_uses_pgvector_text_format() {
        assert_eq!(super::vector_literal(&[0.0, -1.25, 3.5]), "[0,-1.25,3.5]");
    }
}
