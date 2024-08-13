use polars::prelude::*;
use probability::distribution::Binomial;
use probability::prelude::Inverse;
use rand::prelude::*;
use read_write::write_df_to_json;
use read_write::{config_reader::Config, read_df_from_json};

fn get_rand() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    y
}
fn binom_inv(n: f64, p: f64, target_prob: f64) -> f64 {
    Binomial::new(n as usize, p).inverse(target_prob) as f64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_round_5_players_processing_dataframe() {
        // Houston Rockets 1610612745
        let home_team_id = 1610612745;

        // Read players DataFrame
        let config = Config::new();
        let save_path_round_4_players_general = &config.json_data_save_paths_round_4[3];
        let df_players_general = read_df_from_json(save_path_round_4_players_general);

        // calculate x_fga_fta
        let working_roster = df_players_general
            .clone()
            .lazy()
            .filter(col("TEAM_ID").eq(lit(home_team_id))) // Filter by team ID
            .select(&[
                ((col("MIN") + lit(get_rand()) / lit(10000.0)) / lit(48.0)).alias("MP_RATIO"),
                col("*"),
            ])
            .sort(
                ["MP_RATIO"],
                SortMultipleOptions::new().with_order_descending(true),
            )
            .limit(13) // Limit to the top 13 players
            .select(&[
                col("*"),
                (col("FGA") + col("FTA")).alias("FGA_FTA"), // Add 'FGA_FTA' column
            ])
            .with_row_index("RANK", Some(1)) // Adds an 'RANK' column starting from 0
            .select(&[
                col("*"),
                when(col("RANK").gt(lit(5)))
                    .then(col("MIN") - (col("MIN").sum() - lit(240.0)) / lit(8.0))
                    .otherwise(col("MIN"))
                    .alias("xMP"),
            ])
            .select(&[
                col("*"),
                (((lit(0.0182) * col("xMP").pow(lit(2.0)) - lit(0.1186) * col("xMP")
                    + lit(2.3528))
                    / (lit(0.0182) * col("MIN").pow(lit(2.0)) - lit(0.1186) * col("MIN")
                        + lit(2.3528)))
                    * col("FGA_FTA"))
                .alias("xFGA_FTA"),
            ])
            .select(&[
                col("*"),
                (col("FGA") - col("FG3A")).alias("FG2A"),
                (col("FGM") - col("FG3M")).alias("FG2M"),
            ])
            // CALCULATE FG2_PCT.
            .select(&[col("*"), (col("FG2M") / (col("FG2A"))).alias("FG2_PCT")])
            // CALCULATE 2PA POSS.
            .select(&[
                col("*"),
                when((col("FG2A") + col("FG3A") + col("FTA")).eq(lit(0.0)))
                    .then(lit(0.0))
                    .otherwise(col("FG2A") / (col("FG2A") + col("FG3A") + col("FTA")))
                    .alias("2PA_POSS"),
            ])
            // CALCULATE 3PA POSS.
            .select(&[
                col("*"),
                when((col("FG2A") + col("FG3A") + col("FTA")).eq(lit(0.0)))
                    .then(lit(0.0))
                    .otherwise(col("FG3A") / (col("FG2A") + col("FG3A") + col("FTA")))
                    .alias("3PA_POSS"),
            ])
            // CALCULATE FTA POSS.
            .select(&[
                col("*"),
                when((col("FG2A") + col("FG3A") + col("FTA")).eq(lit(0.0)))
                    .then(lit(0.0))
                    .otherwise(col("FTA") / (col("FG2A") + (col("FG3A")) + (col("FTA"))))
                    .alias("FTA_POSS"),
            ])
            .collect()
            .unwrap();

        // println!("{:?}", working_roster.get_column_names());
        // println!("{:?}", working_roster);

        // println!("{:?}", working_roster.get(0).unwrap());

        // for (idx, col) in working_roster.get(0).unwrap().iter().enumerate() {
        //     println!(
        //         "{:?}: {:?} {:?}",
        //         idx,
        //         working_roster.get_column_names()[idx],
        //         col
        //     );
        // }

        let df_len = working_roster.height();
        let mut working_lineup_stats: Vec<Vec<f64>> = Vec::with_capacity(df_len);

        for row_idx in 0..df_len {
            let mut player_stats_row: Vec<f64> = Vec::with_capacity(8);

            let row = working_roster.get(row_idx).unwrap();

            // Create longer-lived references using let bindings
            let player_pts = row[2].try_extract::<f64>().unwrap();
            player_stats_row.push(player_pts);

            let fga_fta = row[17].try_extract::<f64>().unwrap();
            player_stats_row.push(fga_fta);

            let x_fga_fta = row[19].try_extract::<f64>().unwrap();
            player_stats_row.push(x_fga_fta);

            let fg2a_poss = row[22].try_extract::<f64>().unwrap();
            player_stats_row.push(fg2a_poss);

            let fg2_pct = row[25].try_extract::<f64>().unwrap();
            player_stats_row.push(fg2_pct);

            let fg3a_poss = row[23].try_extract::<f64>().unwrap();
            player_stats_row.push(fg3a_poss);

            let fg3_pct = row[16].try_extract::<f64>().unwrap();
            player_stats_row.push(fg3_pct);

            let fta_poss = row[24].try_extract::<f64>().unwrap();
            player_stats_row.push(fta_poss);

            let ft_pct = row[5].try_extract::<f64>().unwrap();
            player_stats_row.push(ft_pct);

            working_lineup_stats.push(player_stats_row);
        }

        let ns = 10_000;
        let mut cum_team_pts_array = Vec::with_capacity(ns);

        let p_start_rng_const = 0.0;
        let p_end_rng_const = 1.0;

        for _ in 0..ns {
            let mut team_pts = 0.0;

            for row in working_lineup_stats.iter() {
                let player_pts = &row[0];
                let fga_fta = &row[1];
                let x_fga_fta = &row[2];

                let fg2a_poss = &row[3];
                let fg2_pct = &row[4];
                if fg2_pct <= &p_start_rng_const
                    || fg2_pct >= &p_end_rng_const
                    || fg2a_poss <= &p_start_rng_const
                    || fg2a_poss >= &p_end_rng_const
                {
                    let sim_pts_result = (x_fga_fta / fga_fta) * player_pts;
                    let rand_sim_pts_result =
                        binom_inv(10000.0, sim_pts_result / 10000.0, get_rand());
                    team_pts += rand_sim_pts_result;
                    continue;
                }

                let fg3a_poss = &row[5];
                let fg3_pct = &row[6];
                if fg3_pct <= &p_start_rng_const
                    || fg3_pct >= &p_end_rng_const
                    || fg3a_poss <= &p_start_rng_const
                    || fg3a_poss >= &p_end_rng_const
                {
                    let sim_pts_result = (x_fga_fta / fga_fta) * player_pts;
                    let rand_sim_pts_result =
                        binom_inv(10000.0, sim_pts_result / 10000.0, get_rand());
                    team_pts += rand_sim_pts_result;
                    continue;
                }

                let fta_poss = &row[7];
                let ft_pct = &row[8];
                if ft_pct <= &p_start_rng_const
                    || ft_pct >= &p_end_rng_const
                    || fta_poss <= &p_start_rng_const
                    || fta_poss >= &p_end_rng_const
                {
                    let sim_pts_result = (x_fga_fta / fga_fta) * player_pts;
                    let rand_sim_pts_result =
                        binom_inv(10000.0, sim_pts_result / 10000.0, get_rand());
                    team_pts += rand_sim_pts_result;
                    continue;
                }

                let rand_1 = get_rand();
                let rand_2 = get_rand();
                let rand_3 = get_rand();
                let rand_4 = get_rand();
                let rand_5 = get_rand();
                let rand_6 = get_rand();
                let rand_7 = get_rand();

                let binom_inv_ns = 10000.0;
                let x_fga_fta_div_10k = *x_fga_fta / binom_inv_ns;
                let x_fga_fta_binom_inv_calc = binom_inv(binom_inv_ns, x_fga_fta_div_10k, rand_1);

                let part_1 = binom_inv(
                    binom_inv(x_fga_fta_binom_inv_calc, *fg2a_poss, rand_2),
                    *fg2_pct,
                    rand_3,
                );
                let part_2 = binom_inv(
                    binom_inv(x_fga_fta_binom_inv_calc, *fg3a_poss, rand_4),
                    *fg3_pct,
                    rand_5,
                );
                let part_3 = binom_inv(
                    binom_inv(x_fga_fta_binom_inv_calc, *fta_poss, rand_6),
                    *ft_pct,
                    rand_7,
                );

                // Calculate the main expression
                let sim_pts_result = 2.0 * part_1 + 3.0 * part_2 + part_3;

                team_pts += sim_pts_result;
                // println!("sim_pts_result: {:?}", sim_pts_result);
            }
            cum_team_pts_array.push(team_pts);
            // println!("team_pts: {:?}", team_pts);
        }
        let cum_team_pts: f64 = cum_team_pts_array.iter().sum();
        println!("cum_team_pts: {:?}", cum_team_pts as i32 / ns as i32);

        write_df_to_json("./test.json", working_roster)
    }

    #[test]
    fn feature_test() {
        let home_team_id = 1610612745;
        // let _away_team_id = 1610612766;

        // Gather team Stats
        let config = Config::new();
        let save_path_final_teams_stats = &config.json_data_save_paths_round_5[0];
        let df_teams_final_stats = read_df_from_json(save_path_final_teams_stats);

        let team_stats = df_teams_final_stats
            .clone()
            .lazy()
            .filter(col("TEAM_ID").eq(lit(home_team_id)))
            .collect()
            .unwrap();

        println!("home_team_pace: {:?}", team_stats);
        // PACE ┆ DEF_RATING ┆ FG2_PCT ┆ FG3_PCT ┆ FT_PCT ┆ TEAM_ID
        // Pace
        let team_pace = team_stats.get(0).unwrap()[0].try_extract::<f64>().unwrap();

        // def_rtg
        let def_rtg = team_stats.get(0).unwrap()[1].try_extract::<f64>().unwrap();

        // fg2_pct
        let opp_fg2_pct = team_stats.get(0).unwrap()[2].try_extract::<f64>().unwrap();

        // fg3_pct
        let opp_fg3_pct = team_stats.get(0).unwrap()[3].try_extract::<f64>().unwrap();

        // ft_pct
        let opp_ft_pct = team_stats.get(0).unwrap()[4].try_extract::<f64>().unwrap();

        println!("home_team_pace: {:?}", team_pace);
        println!("home_team_pace: {:?}", def_rtg);
        println!("home_team_pace: {:?}", opp_fg2_pct);
        println!("home_team_pace: {:?}", opp_fg3_pct);
        println!("home_team_pace: {:?}", opp_ft_pct);
    }
}

// (
// (0.0182*(xMP)^2-0.1186*(xMP)+2.3528)/
// (0.0182*(MP)^2-0.1186*(MP)+2.3528)
// )*"FGA + FTA"
// ((lit(0.0182) * col("xMP") ^ lit(2.0) - lit(0.1186) * col("xMP") + lit(2.3528))/(lit(0.0182) * col("MIN") ^ lit(2.0) - lit(0.1186) * col("MIN") + lit(2.3528)))*col("FGA + FTA"),
//
// 2*BINOM.INV(BINOM.INV(BINOM.INV(10000,xFGA_FTA/10000,RAND()),2PA_POSS,RAND()),FG2_PCT,RAND()) +3*BINOM.INV(BINOM.INV(BINOM.INV(10000,xFGA_FTA/10000,RAND()),3PA_POSS,RAND()),FG3_PCT,RAND()) +BINOM.INV(BINOM.INV(BINOM.INV(10000,xFGA_FTA/10000,RAND()),FTA_POSS,RAND()),FT_PCT,RAND()),ROUND((xFGA_FTA/T4)*PTS)

// 0: "RANK"
// 1: "MP_RATIO"
// 2: "PTS"
// 3: "FTM"
// 4: "FG3M"
// 5: "FT_PCT"
// 6: "FGA"
// 7: "FG3A"
// 8: "TEAM_ID"
// 9: "PLAYER_ID"
// 10: "FG_PCT"
// 11: "FTA"
// 12: "MIN"
// 13: "FGM"
// 14: "GP"
// 15: "PLAYER_NAME"
// 16: "FG3_PCT"
// 17: "FGA_FTA"
// 18: "xMP"
// 19: "xFGA_FTA"
// 20: "FG2A"
// 21: "FG2M"
// 22: "2PA_POSS."
// 23: "3PA_POSS."
// 24: "FTA_POSS."
// 25: "FG2_PCT"
