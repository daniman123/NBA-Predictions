/// Represents the type of entity for the game log: player or team.
enum PlayerOrTeam {
    Player,
    Team,
}

impl From<&str> for PlayerOrTeam {
    fn from(s: &str) -> Self {
        match s {
            "P" => PlayerOrTeam::Player,
            _ => PlayerOrTeam::Team,
        }
    }
}

/// Represents the type of season: regular or playoffs.
enum SeasonType {
    Regular,
    Playoffs,
}

/// Custom trait for converting from a string to `SeasonType`.
trait FromStrOption {
    fn from_str_option(s: &str) -> Option<Self> where Self: Sized;
}

impl FromStrOption for SeasonType {
    fn from_str_option(s: &str) -> Option<Self> {
        match s {
            "Regular Season" => Some(SeasonType::Regular),
            "Playoffs" => Some(SeasonType::Playoffs),
            _ => None,
        }
    }
}

/// Builds the game log endpoint URL for NBA stats.
///
/// # Arguments
///
/// * `player_or_team` - Indicates whether the log is for a player ("P") or team ("T").
/// * `season` - The starting year of the season (e.g., "2023").
/// * `season_type` - The type of season ("Regular Season" or "Playoffs").
///
/// # Returns
///
/// * `Option<String>` - The formatted URL or `None` if inputs are invalid.
pub fn build_game_log_endpoint(
    player_or_team: impl Into<String>,
    season: impl Into<String>,
    season_type: impl Into<String>
) -> Option<String> {
    let player_or_team = match PlayerOrTeam::from(player_or_team.into().as_str()) {
        PlayerOrTeam::Player => "P",
        PlayerOrTeam::Team => "T",
    };

    let season = season.into();
    if season.len() < 4 {
        return None;
    }
    let start_year = &season[0..4];
    let formatted_season = start_year
        .parse::<i32>()
        .ok()
        .map(|year| format!("{}-{}", start_year, year + 1))?;

    let season_type = match SeasonType::from_str_option(season_type.into().as_str()) {
        Some(SeasonType::Regular) => "Regular%20Season",
        Some(SeasonType::Playoffs) => "Playoffs",
        None => {
            return None;
        }
    };

    Some(
        format!(
            "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam={}&Season={}&SeasonType={}&Sorter=DATE",
            player_or_team,
            formatted_season,
            season_type
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_current_regular_url() {
        let endpoint = build_game_log_endpoint("T", "2023", "Regular Season");

        let mock_endpoint =
            "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=T&Season=2023-2024&SeasonType=Regular%20Season&Sorter=DATE";
        assert_eq!(endpoint, Some(mock_endpoint.into()))
    }

    #[test]
    fn test_invalid_season_value_url() {
        let endpoint = build_game_log_endpoint("m", "202a", "Regular Season");

        assert_eq!(endpoint, None)
    }
}
