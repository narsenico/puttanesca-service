use std::sync::Arc;

use crate::{error::Error, hunters::Hunter, Result};

use self::{console::ConsoleProcessor, sqlite::SqliteProcessor};

pub mod console;
pub mod sqlite;

pub trait Processor {
    fn name(&self) -> String;
    fn process(&self, hunter: Arc<dyn Hunter>) -> Result<()>;
}

pub fn create_processor(name: &str) -> Result<Arc<dyn Processor>> {
    if name == "console" {
        return Ok(Arc::new(ConsoleProcessor));
    } else if name.starts_with("sqlite") {
        let db_name = match name.split(':').skip(1).next() {
            Some(name) => name,
            None => Err(Error::ProcessorError(String::from(
                r#"expected "sqlite:<dbname>""#,
            )))?,
        };
        let p = SqliteProcessor::new(String::from(db_name))?;
        return Ok(Arc::new(p));
    }

    Err(Error::HunterNotFound(String::from(name)))
}
