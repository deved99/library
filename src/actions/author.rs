use std::ops::Deref;

use uuid::Uuid;

use crate::db;
use crate::Result;

pub async fn list() -> Result<()> {
    let authors = db::Author::list().await?;
    db::print_table(&authors)?;
    Ok(())
}

pub async fn insert(name: &str) -> Result<()> {
    let author = db::Author::new(name, None).await?;
    println!("{:?}", author);
    Ok(())
}

pub async fn update(
    uuid: Uuid,
    display_name: Option<impl Deref<Target = str>>,
    order_name: Option<impl Deref<Target = str>>,
) -> Result<()> {
    let mut author = db::Author::from_uuid(uuid).await?;
    let mut changed_something = false;
    if let Some(name) = display_name {
        author.update_display_name(&name);
        changed_something = true;
    }
    if let Some(name) = order_name {
        author.update_order_name(&name);
        changed_something = true;
    }
    if changed_something {
        author.save().await?
    }
    Ok(())
}

pub async fn delete(uuid: Uuid) -> Result<()> {
    let author = db::Author::from_uuid(uuid).await?;
    author.delete().await
}
