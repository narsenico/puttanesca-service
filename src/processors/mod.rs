use async_trait::async_trait;
use std::sync::Arc;

use crate::{error::Error, hunters::Hunter, Result};

use self::{console::ConsoleProcessor, sqlite::SqliteProcessor};

mod console;
mod sqlite;

#[async_trait]
pub trait Processor {
    fn name(&self) -> String;
    async fn process(&self, hunter: Arc<dyn Hunter>) -> Result<()>;
}

pub fn create_processor(name: &str) -> Result<Arc<dyn Processor>> {
    if name == "console" {
        return Ok(Arc::new(ConsoleProcessor));
    } else if name.starts_with("sqlite") {
        let db_name = match name.split(':').nth(1) {
            Some(name) => name,
            None => Err(Error::ProcessorError(String::from(
                r#"expected "sqlite:<dbname>""#,
            )))?,
        };
        let p = SqliteProcessor::new(String::from(db_name))?;
        return Ok(Arc::new(p));
    }

    Err(Error::ProcessorNotFound(String::from(name)))
}
