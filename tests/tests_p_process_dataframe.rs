mod support;
use polars::{ frame::UniqueKeepStrategy, prelude::{ DataFrameJoinOps, JoinArgs } };
use read_write::Config;
use support::utils::write_df_to_json;
use crate::support::utils::read_df_from_json;

#[test]
pub fn clean_data_test() {
    // let path = "../data/json_data/for_preprocessing_test_output.json";
    
    let config = Config::new();
    let mut df = read_df_from_json(&config.input_path);

    // println!("{:?}", df.get_column_names());
    // println!("{:?}", df);

    df = df.drop_many(&["PLUS_MINUS", "VIDEO_AVAILABLE"]);
    println!("{:?}", df.get_column_names());
    println!("TYPES {:?}", df.dtypes());

    df = df.sort(["GAME_ID"], Default::default()).unwrap();
    println!("{:?}", df.select(["GAME_ID"]));

    let unique_game_id_first = df
        .unique(Some(&["GAME_ID".to_owned()]), UniqueKeepStrategy::First, None)
        .unwrap();
    let unique_game_id_last = df
        .unique(Some(&["GAME_ID".to_owned()]), UniqueKeepStrategy::Last, None)
        .unwrap();

    let games = unique_game_id_first
        .join(&unique_game_id_last, ["GAME_ID"], ["GAME_ID"], JoinArgs::default())
        .unwrap();

    println!("{:?}", games.get_column_names());
    println!("{:?}", games.select(["MATCHUP", "MATCHUP_right"]));
    println!("{:?}", games);

    // let path = "../data/processed_data/json_data/game_matchups.json";
    write_df_to_json(&config.output_path, games);
}
