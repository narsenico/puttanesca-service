use std::sync::Arc;

use crate::{error::Error, Result};

use self::test_hunter::TestHunter;

pub mod test_hunter;

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

pub trait Hunter: Send + Sync {
    fn name(&self) -> String;
    fn find_matches(&self) -> Result<Vec<Match>>;
}

pub fn create_hunter(name: &str) -> Result<Arc<dyn Hunter>> {
    if name == "test" {
        return Ok(Arc::new(TestHunter));
    }

    Err(Error::HunterNotFound(String::from(name)))
}
