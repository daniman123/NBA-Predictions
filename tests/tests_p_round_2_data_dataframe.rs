use read_write::read_df_from_json;

#[test]
fn test_round_2_data() {
    let path =
        "../data/round_2_data/json_data/team_stats/teams_general_advanced/per_game/2023_24/teams_general_advanced_per_game_2023_24.json";
    let df = read_df_from_json(path);

    println!("{:?}", df.get_column_names().iter());
}
