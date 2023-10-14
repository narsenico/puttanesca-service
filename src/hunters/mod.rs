use async_trait::async_trait;
use std::sync::Arc;

use crate::models::Match;
use crate::{error::Error, Result};

use self::sky::SkyHunter;
use self::test::TestHunter;

mod sky;
mod test;

#[async_trait]
pub trait Hunter: Send + Sync {
    fn name(&self) -> String;
    async fn find_matches(&self) -> Result<Vec<Match>>;
}

pub fn create_hunter(name: &str) -> Result<Arc<dyn Hunter>> {
    if name == "test" {
        return Ok(Arc::new(TestHunter));
    }

    if name == "sky" {
        return Ok(Arc::new(SkyHunter::new()?));
    }

    Err(Error::HunterNotFound(String::from(name)))
}
