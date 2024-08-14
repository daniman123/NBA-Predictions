use crate::Result;
use polars::prelude::*;
use read_write::write_df_to_json;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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

pub fn extract_result_sets_json_data(result_sets_json_data: Value, path: &str) -> Result<()> {
    let json_data = serde_json::from_value::<NbaApiResultSetsJsonBody>(result_sets_json_data)?;
    let result_sets = &json_data.resultSets[0];
    let headers = &result_sets.headers;
    let rows = &result_sets.rowSet;
    let series_data = json_to_series(headers, rows);
    let df_result_sets_data =
        DataFrame::new(series_data).expect("DataFrame creation should succeed");

    write_df_to_json(path, df_result_sets_data);
    Ok(())
}
