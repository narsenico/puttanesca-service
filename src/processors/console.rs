use crate::Result;
use async_trait::async_trait;

use super::Processor;

pub struct ConsoleProcessor;

#[async_trait]
impl Processor for ConsoleProcessor {
    fn name(&self) -> String {
        String::from("Console Processor")
    }

    async fn process(&self, hunter: std::sync::Arc<dyn crate::hunters::Hunter>) -> Result<()> {
        let matches = hunter.find_matches().await?;
        for m in matches {
            println!("{}", m);
        }

        Ok(())
    }
}
