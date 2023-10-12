use async_trait::async_trait;
use std::fmt::Display;
use std::sync::Arc;

use crate::{error::Error, Result};

use self::sky::SkyHunter;
use self::test::TestHunter;

mod sky;
mod test;

// TODO: manage date and time format (with struct?)

#[derive(Debug, Default)]
pub struct Match {
    pub index: usize,
    pub date: String,
    pub team1: String,
    pub team2: String,
    pub team1_goals: Option<usize>,
    pub team2_goals: Option<usize>,
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Match #{} {}: {} vs {} ({} - {})",
            self.index,
            self.date,
            self.team1,
            self.team2,
            self.team1_goals
                .map(|g| g.to_string())
                .unwrap_or("?".to_string()),
            self.team2_goals
                .map(|g| g.to_string())
                .unwrap_or("?".to_string()),
        )
    }
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
