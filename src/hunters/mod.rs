use async_trait::async_trait;
use std::sync::Arc;

use crate::{error::Error, Result};

use self::sky::SkyHunter;
use self::test::TestHunter;

mod sky;
mod test;

#[derive(Debug)]
pub struct MatchDay {
    pub index: usize,
    pub date: String,
}

#[derive(Debug)]
pub struct Match {
    pub day: MatchDay,
    pub team1: String,
    pub team2: String,
}

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
