use crate::db::{get_pool, AsRow, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(PartialEq, Eq, Hash, Clone, Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct ReadingList {
    uuid: Uuid,
    title: String,
    date_started: Option<NaiveDate>,
    date_finished: Option<NaiveDate>,
    display_authors: Vec<String>,
    order_authors: Vec<String>,
    tags: Vec<String>,
}
impl ReadingList {
    pub async fn get() -> Result<Vec<Self>> {
        let db = get_pool().await?;
        let results: Vec<Self> = sqlx::query_as_unchecked!(
            Self,
            "SELECT uuid, title, tags,
                    date_started, date_finished,
                    display_authors, order_authors
             FROM reading_list"
        )
        .fetch_all(db)
        .await?;
        Ok(results)
    }
}

impl AsRow for ReadingList {
    fn titles() -> Vec<String> {
        ["title", "authors", "state", "tags"]
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
    fn columns(&self) -> Vec<String> {
        let state = match (self.date_started, self.date_finished) {
            (Some(_), Some(_)) => "finished",
            (Some(_), None) => "reading",
            (None, None) => "unread",
            (None, Some(_)) => {
                log::error!("{:?}", self);
                "Book finished without a start date"
            }
        }
        .to_string();
        vec![
            format!("{}", self.title),
            format!("{}", self.display_authors.join("; ")),
            format!("{}", state),
            format!("{}", self.tags.join("; ")),
        ]
    }
}
