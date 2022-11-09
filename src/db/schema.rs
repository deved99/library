use super::get_pool;
use crate::Result;
use sqlx::{self, Executor};

pub async fn reset() -> Result<()> {
    let db = get_pool().await?;
    let query = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/schema.sql"));
    db.execute(query).await?;
    Ok(())
}
