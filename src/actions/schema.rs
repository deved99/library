use crate::{db, Error, Result};

pub async fn reset(force: bool) -> Result<()> {
    if force {
        db::schema::reset().await
    } else {
        log::error!("You are not sure enough. See the help page.");
        Err(Error::MissingArgument {
            expected: "force".to_string(),
        })
    }
}
