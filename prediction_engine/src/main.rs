use polars::prelude::*;
use probability::distribution::Binomial;
use probability::prelude::Inverse;
use rand::prelude::*;
use rayon::prelude::*;
use read_write::{config_reader::Config, read_df_from_json};

#[derive(Default, Debug)]
pub struct TeamStats {
    pub team_pace: f64,
    pub pace_dif: Option<f64>,
    pub def_rtg: f64,
    pub opp_def_rtg: Option<f64>,
    pub opp_fg2_pct: f64,
    pub opp_fg3_pct: f64,
    pub opp_ft_pct: f64,
}

fn get_team_stats(team_id: i32) -> TeamStats {
    // Gather team Stats
    let config = Config::new();
    let save_path_final_teams_stats = &config.json_data_save_paths_round_5[0];
    let df_teams_final_stats = read_df_from_json(save_path_final_teams_stats);

    let team_stats = df_teams_final_stats
        .clone()
        .lazy()
        .filter(col("TEAM_ID").eq(lit(team_id)))
        .collect()
        .unwrap();

    // Pace
    let team_pace = team_stats.get(0).unwrap()[0].try_extract::<f64>().unwrap();

    // def_rtg
    let def_rtg = team_stats.get(0).unwrap()[7].try_extract::<f64>().unwrap();
    // fg2_pct
    let opp_fg2_pct = team_stats.get(0).unwrap()[8].try_extract::<f64>().unwrap();
    // fg3_pct
    let opp_fg3_pct = team_stats.get(0).unwrap()[9].try_extract::<f64>().unwrap();

    // ft_pct
    let opp_ft_pct = team_stats.get(0).unwrap()[10].try_extract::<f64>().unwrap();

    TeamStats {
        team_pace,
        def_rtg,
        opp_fg2_pct,
        opp_fg3_pct,
        opp_ft_pct,
        ..Default::default()
    }
}

fn get_rand() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    y
}

fn binom_inv(n: f64, p: f64, target_prob: f64) -> f64 {
    Binomial::new(n as usize, p).inverse(target_prob) as f64
}

fn read_player_stats() -> DataFrame {
    // Read players DataFrame
    let config = Config::new();
    let save_path_round_4_players_general = &config.json_data_save_paths_round_4[3];
    read_df_from_json(save_path_round_4_players_general)
}

fn aggregate_working_roster(
    df_players_general: DataFrame,
    team_id: i32,
    team_stats: TeamStats,
) -> DataFrame {
    // calculate x_fga_fta
    let mut working_stats = df_players_general
        .clone()
        .lazy()
        .filter(col("TEAM_ID").eq(lit(team_id))) // Filter by team ID
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
            (((((lit(0.0182) * col("xMP").pow(lit(2.0)) - lit(0.1186) * col("xMP")
                + lit(2.3528))
                / (lit(0.0182) * col("MIN").pow(lit(2.0)) - lit(0.1186) * col("MIN")
                    + lit(2.3528)))
                * col("FGA_FTA"))
                * lit(team_stats.def_rtg))
                * lit(team_stats.pace_dif.unwrap()))
            .alias("xFGA_FTA"),
        ])
        .select(&[
            col("*"),
            (col("FGA") - col("FG3A")).alias("FG2A"),
            (col("FGM") - col("FG3M")).alias("FG2M"),
        ])
        // CALCULATE FG2_PCT.
        .select(&[
            col("*"),
            ((col("FG2M") / (col("FG2A"))) * lit(team_stats.opp_fg2_pct)).alias("FG2_PCT"),
        ])
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

    working_stats
        .apply("FG2_PCT", |s| {
            s.iter()
                .map(|val| val.try_extract::<f64>().unwrap() * team_stats.opp_fg2_pct)
                .collect::<Series>()
        })
        .unwrap();
    working_stats
        .apply("FG3_PCT", |s| {
            s.iter()
                .map(|val| val.try_extract::<f64>().unwrap() * team_stats.opp_fg3_pct)
                .collect::<Series>()
        })
        .unwrap();
    working_stats
        .apply("FT_PCT", |s| {
            s.iter()
                .map(|val| val.try_extract::<f64>().unwrap() * team_stats.opp_ft_pct)
                .collect::<Series>()
        })
        .unwrap();

    working_stats
}

fn get_lineup_stats(working_roster: DataFrame) -> Vec<Vec<f64>> {
    let df_len = working_roster.height();
    let mut working_lineup_stats: Vec<Vec<f64>> = Vec::with_capacity(df_len);

    for row_idx in 0..df_len {
        let row = working_roster.get(row_idx).unwrap();

        // Preallocate a vector with the exact size to avoid push overhead
        let mut player_stats_row: Vec<f64> = vec![0.0; 8];

        // Directly access and assign values
        player_stats_row[0] = row[2].try_extract::<f64>().unwrap();
        player_stats_row[1] = row[17].try_extract::<f64>().unwrap();
        player_stats_row[2] = row[19].try_extract::<f64>().unwrap();
        player_stats_row[3] = row[22].try_extract::<f64>().unwrap();
        player_stats_row[4] = row[25].try_extract::<f64>().unwrap();
        player_stats_row[5] = row[23].try_extract::<f64>().unwrap();
        player_stats_row[6] = row[16].try_extract::<f64>().unwrap();
        player_stats_row[7] = row[24].try_extract::<f64>().unwrap();
        player_stats_row.push(row[5].try_extract::<f64>().unwrap());

        working_lineup_stats.push(player_stats_row);
    }

    working_lineup_stats
}

fn predict_player_pts(
    row: &[f64],
    binom_inv_ns: f64,
    p_start_rng_const: f64,
    p_end_rng_const: f64,
) -> f64 {
    let player_pts = row[0];
    let fga_fta = row[1];
    let x_fga_fta = row[2];

    // Precompute this as it's used multiple times
    let x_fga_fta_div_10k = x_fga_fta / binom_inv_ns;

    // Exit early if any percentage is out of range
    let out_of_range = |val: f64| -> bool { val <= p_start_rng_const || val >= p_end_rng_const };

    if out_of_range(row[4])
        || out_of_range(row[3])
        || out_of_range(row[6])
        || out_of_range(row[5])
        || out_of_range(row[8])
        || out_of_range(row[7])
    {
        let sim_pts_result = (x_fga_fta / fga_fta) * player_pts;
        return binom_inv(binom_inv_ns, sim_pts_result / binom_inv_ns, get_rand());
        
    }

    // Precompute random numbers
    let rand_vals: [f64; 7] = [
        get_rand(),
        get_rand(),
        get_rand(),
        get_rand(),
        get_rand(),
        get_rand(),
        get_rand(),
    ];

    let x_fga_fta_binom_inv_calc = binom_inv(binom_inv_ns, x_fga_fta_div_10k, rand_vals[0]);

    // Calculate parts
    let part_1 = binom_inv(
        binom_inv(x_fga_fta_binom_inv_calc, row[3], rand_vals[1]),
        row[4],
        rand_vals[2],
    );
    let part_2 = binom_inv(
        binom_inv(x_fga_fta_binom_inv_calc, row[5], rand_vals[3]),
        row[6],
        rand_vals[4],
    );
    let part_3 = binom_inv(
        binom_inv(x_fga_fta_binom_inv_calc, row[7], rand_vals[5]),
        row[8],
        rand_vals[6],
    );

    // Calculate the main expression
    2.0 * part_1 + 3.0 * part_2 + part_3
}

fn sim_team_pts(working_lineup_stats: &[Vec<f64>]) -> f64 {
    let mut team_pts = 0.0;
    let p_start_rng_const = 0.0;
    let p_end_rng_const = 1.0;
    let binom_inv_ns = 10000.0;

    for row in working_lineup_stats.iter() {
        let simulated_player_pts =
            predict_player_pts(row, binom_inv_ns, p_start_rng_const, p_end_rng_const);
        team_pts += simulated_player_pts
    }

    team_pts
}

fn main() {
    // Indiana Pacers	1610612754
    let home_team_id = 1610612754;
    // Chicago Bulls	1610612741
    let away_team_id = 1610612741;

    // Gather team Stats
    let mut home_team_stats = get_team_stats(away_team_id);
    let mut away_team_stats = get_team_stats(home_team_id);

    let total_est_pace = (home_team_stats.team_pace + away_team_stats.team_pace) / 2.0;

    let home_pace_dif =
        (home_team_stats.team_pace * total_est_pace).sqrt() / home_team_stats.team_pace;
    let away_pace_dif =
        (away_team_stats.team_pace * total_est_pace).sqrt() / away_team_stats.team_pace;

    home_team_stats.pace_dif = Some(away_pace_dif);
    away_team_stats.pace_dif = Some(home_pace_dif);

    // Gather players Stats
    let df_players_general = read_player_stats();

    // HOME_TEAM: Calculate stats
    let working_roster_home =
        aggregate_working_roster(df_players_general.clone(), home_team_id, home_team_stats);
    let working_lineup_stats_home = get_lineup_stats(working_roster_home);

    // AWAY_TEAM: Calculate stats
    let working_roster_away =
        aggregate_working_roster(df_players_general, away_team_id, away_team_stats);
    let working_lineup_stats_away = get_lineup_stats(working_roster_away);

    let ns = 100_000;
    // let ns = 100;

    // Pre-allocate memory
    let mut home_team_pts_array = vec![0.0; ns];
    let mut away_team_pts_array = vec![0.0; ns];

    // Collect results into a vector of tuples
    let results: Vec<(f64, f64)> = (0..ns)
        .into_par_iter()
        .map(|_i| {
            let team_pts_home = sim_team_pts(&working_lineup_stats_home);
            let team_pts_away = sim_team_pts(&working_lineup_stats_away);
            (team_pts_home, team_pts_away)
        })
        .collect();

    // Update the arrays based on the collected results
    for (i, (team_pts_home, team_pts_away)) in results.iter().enumerate() {
        home_team_pts_array[i] = *team_pts_home;
        away_team_pts_array[i] = *team_pts_away;
    }

    // Calculate win counts
    let (home_team_wins_count, away_team_wins_count, draws_count) = results
        .into_iter()
        .map(|(team_pts_home, team_pts_away)| {
            match team_pts_home.partial_cmp(&team_pts_away).unwrap() {
                std::cmp::Ordering::Greater => (1.0, 0.0, 0.0),
                std::cmp::Ordering::Less => (0.0, 1.0, 0.0),
                std::cmp::Ordering::Equal => (0.0, 0.0, 1.0),
            }
        })
        .fold((0.0, 0.0, 0.0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2)
        });

    // Calculate cumulative points
    let home_cum_team_pts: f64 = home_team_pts_array.iter().sum();
    let away_cum_team_pts: f64 = away_team_pts_array.iter().sum();

    // Calculate average points
    let avg_home_team_pts = home_cum_team_pts / ns as f64;
    let avg_away_team_pts = away_cum_team_pts / ns as f64;
    println!("avg_home_team_pts: {:?}", avg_home_team_pts);
    println!("avg_away_team_pts: {:?}", avg_away_team_pts);

    // Calculate win percentages
    let home_team_win_pct = home_team_wins_count / ns as f64;
    let away_team_win_pct = away_team_wins_count / ns as f64;
    let draw_pct = draws_count / ns as f64;
    println!("home_team_win_pct: {:.2}%", home_team_win_pct * 100.0);
    println!("away_team_win_pct: {:.2}%", away_team_win_pct * 100.0);
    println!("draw_pct: {:.2}%", draw_pct * 100.0);
}

// // Pre-allocate memory
// let mut home_team_pts_array = Vec::with_capacity(ns);
// let mut away_team_pts_array = Vec::with_capacity(ns);

// let mut home_team_wins_count = 0.0;
// let mut away_team_wins_count = 0.0;
// let mut draws_count = 0.0;

// // Consider parallelizing this loop if applicable
// for _ in 0..ns {
//     let team_pts_home = sim_pts(&working_lineup_stats_home);
//     let team_pts_away = sim_pts(&working_lineup_stats_away);

//     home_team_pts_array.push(team_pts_home);
//     away_team_pts_array.push(team_pts_away);

//     match team_pts_home.partial_cmp(&team_pts_away).unwrap() {
//         std::cmp::Ordering::Greater => home_team_wins_count += 1.0,
//         std::cmp::Ordering::Less => away_team_wins_count += 1.0,
//         std::cmp::Ordering::Equal => draws_count += 1.0,
//     }
// }

// let (home_team_wins_count, away_team_wins_count, draws_count): (f64, f64, f64) = (0..ns)
//     .into_par_iter()
//     .map(|_| {
//         let team_pts_home = sim_pts(&working_lineup_stats_home);
//         let team_pts_away = sim_pts(&working_lineup_stats_away);

//         let mut home_wins = 0.0;
//         let mut away_wins = 0.0;
//         let mut draws = 0.0;

//         match team_pts_home.partial_cmp(&team_pts_away).unwrap() {
//             std::cmp::Ordering::Greater => home_wins += 1.0,
//             std::cmp::Ordering::Less => away_wins += 1.0,
//             std::cmp::Ordering::Equal => draws += 1.0,
//         }

//         (home_wins, away_wins, draws)
//     })
//     .reduce(
//         || (0.0, 0.0, 0.0),
//         |(hwc1, awc1, dc1), (hwc2, awc2, dc2)| (hwc1 + hwc2, awc1 + awc2, dc1 + dc2),
//     );

// let ns = 100_000;

//     // Pre-allocate memory
//     let mut home_team_pts_array = vec![0.0; ns];
//     let mut away_team_pts_array = vec![0.0; ns];

//     let (home_team_wins_count, away_team_wins_count, draws_count) = (0..ns).into_par_iter().map(|i| {
//         let team_pts_home = sim_pts(&working_lineup_stats_home);
//         let team_pts_away = sim_pts(&working_lineup_stats_away);

//         home_team_pts_array[i] = team_pts_home;
//         away_team_pts_array[i] = team_pts_away;

//         match team_pts_home.partial_cmp(&team_pts_away).unwrap() {
//             std::cmp::Ordering::Greater => (1.0, 0.0, 0.0),
//             std::cmp::Ordering::Less => (0.0, 1.0, 0.0),
//             std::cmp::Ordering::Equal => (0.0, 0.0, 1.0),
//         }
//     }).reduce(|| (0.0, 0.0, 0.0), |acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2));

//     // Calculate cumulative points
//     let home_cum_team_pts: f64 = home_team_pts_array.iter().sum();
//     let away_cum_team_pts: f64 = away_team_pts_array.iter().sum();

//     // Calculate average points
//     let avg_home_team_pts = home_cum_team_pts / ns as f64;
//     let avg_away_team_pts = away_cum_team_pts / ns as f64;
//     println!("avg_home_team_pts: {:?}", avg_home_team_pts);
//     println!("avg_away_team_pts: {:?}", avg_away_team_pts);

//     // Calculate win percentages
//     let home_team_win_pct = home_team_wins_count / ns as f64;
//     let away_team_win_pct = away_team_wins_count / ns as f64;
//     let draw_pct = draws_count / ns as f64;
//     println!("home_team_win_pct: {:.2}%", home_team_win_pct * 100.0);
//     println!("away_team_win_pct: {:.2}%", away_team_win_pct * 100.0);
//     println!("draw_pct: {:.2}%", draw_pct * 100.0);
