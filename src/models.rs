use std::fmt::Display;

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
