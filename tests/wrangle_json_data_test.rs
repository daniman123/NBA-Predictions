use polars::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fs;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct NbaApiResultSetsJsonBody {
    parameters: Map<String, Value>,
    resource: Value,
    resultSets: Vec<ResultSets>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct ResultSets {
    pub headers: Vec<String>,
    name: Value,
    pub rowSet: Vec<Vec<Value>>,
}

// Function to convert headers and rows into a Vec<Series>
fn json_to_series(headers: &[String], rows: &[Vec<Value>]) -> Vec<Series> {
    headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let column_data: Vec<AnyValue> = rows
                .iter()
                .map(|row| match &row[i] {
                    Value::Number(n) => {
                        if n.is_i64() {
                            AnyValue::Int64(n.as_i64().unwrap_or(0))
                        } else {
                            AnyValue::Float64(n.as_f64().unwrap_or(0.0))
                        }
                    }
                    Value::String(s) => AnyValue::String(s),
                    _ => AnyValue::Null,
                })
                .collect();
            Series::new(&header.to_string(), column_data)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn json_data_test() {
        // let path = "../data/json_data/test_output.json";
        let path = "../data/raw_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json";
        let file = fs::File::open(path).expect("file should open read only");
        let json_data: serde_json::Value = serde_json::from_reader(file).unwrap();
        let result_sets = json_data.as_object().unwrap().get("resultSets").unwrap();

        let headers = result_sets[0].get("headers").unwrap();
        let rows = result_sets[0].get("rowSet").unwrap();

        let series: Vec<Series> = headers
            .as_array()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(i, header)| {
                let column_data: Vec<AnyValue> = rows
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|row| match &row[i] {
                        Value::Number(n) => AnyValue::Int32(n.as_i64().unwrap_or(0) as i32),
                        Value::String(s) => AnyValue::String(s),
                        _ => AnyValue::Null,
                    })
                    .collect();
                Series::new(header.as_str().unwrap(), column_data)
            })
            .collect();

        // Create DataFrame
        let df = DataFrame::new(series).unwrap();

        // Print DataFrame
        println!("{:?}", df);

        // let mut file =
        //     std::fs::File::create("../data/json_data/for_preprocessing_test_output.json").unwrap();

        // JsonWriter::new(&mut file)
        //     .with_json_format(JsonFormat::Json)
        //     .finish(&mut df)
        //     .unwrap();
    }
    
    #[test]
    fn test_name() {
        let path = "../data/raw_data/json_data/team_stats/opponent_shooting_general/per_game/2023_24/opponent_shooting_general_per_game_2023_24.json";
        let file = fs::File::open(path).expect("file should open read only");
        let json_data_serde_value: serde_json::Value = serde_json::from_reader(file).unwrap();

        let json_data = serde_json::from_value::<NbaApiResultSetsJsonBody>(json_data_serde_value).unwrap();
        let result_sets = &json_data.resultSets[0];
        let headers = &result_sets.headers;
        let rows = &result_sets.rowSet;
        let series_data = json_to_series(headers, rows);

        let df_result_sets_data =
        DataFrame::new(series_data).expect("DataFrame creation should succeed");
        println!("{:?}", df_result_sets_data);

    }
}
