use crate::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchDate {
    year: String,
    month: String,
    day: String,
}

impl MatchDate {
    pub fn new(year: impl Into<String>, month: impl Into<String>, day: impl Into<String>) -> Self {
        MatchDate {
            year: year.into(),
            month: month.into(),
            day: day.into(),
        }
    }

    pub fn from(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        let tokens = value.split('-').collect::<Vec<&str>>();
        let year = tokens.first()?.to_string();
        let month = tokens.get(1)?.to_string();
        let day = tokens.get(2)?.to_string();

        Some(Self {
            year,
            month,
            day
        })
    }
}

impl ToString for MatchDate {
    fn to_string(&self) -> String {
        format!("{:>4}-{:0>2}-{:0>2}", self.year, self.month, self.day)
    }
}

#[derive(Debug, PartialEq)]
pub struct Match {
    pub(crate) match_day: usize,
    pub(crate) match_date: MatchDate,
    pub(crate) team1: String,
    pub(crate) team2: String,
    pub(crate) team1_score: Option<usize>,
    pub(crate) team2_score: Option<usize>,
    pub(crate) team1_icon: Option<String>,
    pub(crate) team2_icon: Option<String>,
}

impl Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Match #{} {}: {} vs {} ({} - {})",
            self.match_day,
            self.match_date.to_string(),
            self.team1,
            self.team2,
            self.team1_score
                .map(|g| g.to_string())
                .unwrap_or("?".to_string()),
            self.team2_score
                .map(|g| g.to_string())
                .unwrap_or("?".to_string()),
        )
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use crate::models::MatchDate;

    #[test]
    fn match_date_to_string_success() {
        // Assert
        let year = "2000".to_string();
        let month = "2".to_string();
        let day = "1".to_string();
        let match_date = MatchDate { year, month, day };

        // Act
        let s = match_date.to_string();

        // Assert
        assert_eq!("2000-02-01", s)
    }
}
