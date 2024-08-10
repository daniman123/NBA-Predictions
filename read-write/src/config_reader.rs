use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Deserialize, Default)]
pub struct ApiEndpoints {
    pub team_stats_advanced: String,
    pub opponent_shooting: String,
    pub team_stats_base: String,
    pub player_averages: String,
}

#[derive(Deserialize, Default)]
pub struct Config {
    pub api_endpoints: ApiEndpoints,
    pub bin_data_save_paths: Vec<String>,
    pub json_data_save_paths: Vec<String>,
    pub json_data_save_paths_round_2: Vec<String>,
    pub json_data_save_paths_round_3: Vec<String>,
    pub json_data_save_paths_round_4: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let mut file = File::open("../data/config.json").expect("Config file not found");
        let mut data = String::new();
        file.read_to_string(&mut data)
            .expect("Failed to read config file");
        serde_json::from_str::<Config>(&data).expect("Failed to parse config file")
    }
}

// let api_endpoints = [
//     "https://stats.nba.com/stats/leaguedashteamstats?Conference=&DateFrom=&DateTo=&Division=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Advanced&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&TwoWay=0&VsConference=&VsDivision=",
//     "https://stats.nba.com/stats/leaguedashoppptshot?Conference=&DateFrom=&DateTo=&Division=&GameSegment=&GeneralRange=Overall&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&TeamID=0&VsConference=&VsDivision=",
//     "https://stats.nba.com/stats/leaguedashteamstats?Conference=&DateFrom=&DateTo=&Division=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&TwoWay=0&VsConference=&VsDivision=",
// ];

// let bin_data_save_path = [
//     "../data/raw_data/binary_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.bin",
//     "../data/raw_data/binary_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.bin",
//     "../data/raw_data/binary_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.bin",
// ];

// let json_data_save_path = [
//     "../data/round_1_data/json_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.json",
//     "../data/round_1_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json",
//     "../data/round_1_data/json_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.json",
// ];

// let json_data_save_path_round_2 = [
//     "../data/round_2_data/json_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.json",
//     "../data/round_2_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json",
//     "../data/round_2_data/json_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.json",
// ];
// ...
// ...