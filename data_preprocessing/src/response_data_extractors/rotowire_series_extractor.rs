use crate::Result;
use polars::prelude::*;
use read_write::write_df_to_json;
use serde::Deserialize;
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct RotowireData {
    ID: Value,
    URL: Value,
    firstname: Value,
    lastname: Value,
    player: Value,
    team: Value,
    position: Value,
    injury: Value,
    status: Value,
    rDate: Value,
}
type RotowireApiJsonBody = Vec<RotowireData>;
// [{"ID":"3445","URL":"\/basketball\/player\/steven-adams-3445","firstname":"Steven","lastname":"Adams","player":"Steven Adams","team":"HOU","position":"C","injury":"Knee","status":"Game Time Decision","rDate":"<i>Subscribers Only<\/i>"},

pub fn extract_rotowire_series_data(
    rotowire_series_json_data: Value,
    path: &str,
) -> Result<DataFrame> {
    let json_data = serde_json::from_value::<RotowireApiJsonBody>(rotowire_series_json_data)?;

    // Create a DataFrame with columns
    let df_injury_report = DataFrame::new(vec![
        Series::new(
            "ID",
            &json_data
                .iter()
                .map(|row| row.ID.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "URL",
            &json_data
                .iter()
                .map(|row| row.URL.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "firstname",
            &json_data
                .iter()
                .map(|row| row.firstname.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "lastname",
            &json_data
                .iter()
                .map(|row| row.lastname.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "player",
            &json_data
                .iter()
                .map(|row| row.player.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "team",
            &json_data
                .iter()
                .map(|row| row.team.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "position",
            &json_data
                .iter()
                .map(|row| row.position.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "injury",
            &json_data
                .iter()
                .map(|row| row.injury.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "status",
            &json_data
                .iter()
                .map(|row| row.status.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        Series::new(
            "r_date",
            &json_data
                .iter()
                .map(|row| row.rDate.as_str().unwrap())
                .collect::<Vec<&str>>(),
        ),
        // Add other columns as needed
    ])
    .unwrap();

    write_df_to_json(path, df_injury_report.clone());

    Ok(df_injury_report.clone())
}
