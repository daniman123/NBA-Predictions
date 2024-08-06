mod error;
pub mod decompressed_json_reader;
pub mod game_log_json_data_into_data_frame;
pub mod utils;
mod fetch_endpoint_response;
use decompressed_json_reader::read_decompressed_json;
use fetch_endpoint_response::get_response_bytes;
use game_log_json_data_into_data_frame::read_json_to_df;
use utils::read_write_from_file_tools::write_json_to_file;

pub use self::error::{ Error, Result };

#[tokio::main]
async fn main() -> Result<()> {
    let api_endpoints = [
        "https://stats.nba.com/stats/leaguedashteamstats?Conference=&DateFrom=&DateTo=&Division=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Advanced&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&TwoWay=0&VsConference=&VsDivision=",
        "https://stats.nba.com/stats/leaguedashoppptshot?Conference=&DateFrom=&DateTo=&Division=&GameSegment=&GeneralRange=Overall&LastNGames=0&LeagueID=00&Location=&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&TeamID=0&VsConference=&VsDivision=",
        "https://stats.nba.com/stats/leaguedashteamstats?Conference=&DateFrom=&DateTo=&Division=&GameScope=&GameSegment=&Height=&ISTRound=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerExperience=&PlayerPosition=&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&StarterBench=&TeamID=0&TwoWay=0&VsConference=&VsDivision=",
    ];

    let bin_data_save_path = [
        "../data/pre_processed_data/binary_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.bin",
        "../data/pre_processed_data/binary_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.bin",
        "../data/pre_processed_data/binary_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.bin",
    ];

    let json_data_save_path = [
        "../data/pre_processed_data/json_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.json",
        "../data/pre_processed_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json",
        "../data/pre_processed_data/json_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.json",
    ];

    let json_data_save_path_round_2 = [
        "../data/processed_data/json_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.json",
        "../data/processed_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json",
        "../data/processed_data/json_data/team_stats/teams_general_opponent/per_game/2023_24/teams_general_opponent_per_game_2023_24.json",
    ];

    for (index, api_endpoint) in api_endpoints.iter().enumerate() {
        let bin_save_path = bin_data_save_path[index];
        let json_save_path = json_data_save_path[index];
        let json_save_path_round_2 = json_data_save_path_round_2[index];

        get_response_bytes(api_endpoint, bin_save_path).await?;
        let json_data = read_decompressed_json(bin_save_path).unwrap();
        write_json_to_file(json_save_path, json_data).unwrap();
        read_json_to_df(json_save_path, json_save_path_round_2);
    }

    Ok(())
}
