use crate::Result;

use super::Processor;

pub struct ConsoleProcessor;

impl Processor for ConsoleProcessor {
    fn name(&self) -> String {
        String::from("Console Processor")
    }

    fn process(&self, hunter: std::sync::Arc<dyn crate::hunters::Hunter>) -> Result<()> {
        let matches = hunter.find_matches().unwrap();
        for m in matches {
            println!("Match #{} {}: {} vs {}", m.day.index, m.day.date, m.team1, m.team2);
        }

        Ok(())
    }
}
