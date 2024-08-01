use polars::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use crate::Result;

/// Reads a JSON file from the specified path and parses it into a `serde_json::Value`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the JSON file.
///
/// # Returns
///
/// * `Result<Value, Box<dyn std::error::Error>>` - On success, returns the parsed JSON data as a `serde_json::Value`.
///   On failure, returns an error.
///
/// # Errors
///
/// This function will return an error if:
///
/// * The file at the specified path cannot be opened.
/// * The file's content cannot be parsed as valid JSON.
///
/// # Examples
///
/// ```
/// use serde_json::Value;
///
/// let path = "../data/json_data/test_output.json";
/// match read_json_file(path) {
///     Ok(json_data) => {
///         // Now you can work with the parsed JSON data.
///     },
///     Err(e) => {
///         eprintln!("Error reading JSON file: {}", e);
///     }
/// }
/// ```
fn read_json_file(path: &str) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json_data = serde_json::from_reader(reader)?;
    Ok(json_data)
}

/// Extracts headers and rows from the given JSON data.
///
/// This function expects the JSON data to have a specific structure where
/// the "resultSets" key maps to an array of objects, each containing "headers"
/// and "rowSet" keys. The "headers" key should map to an array of strings,
/// and the "rowSet" key should map to an array of arrays, each representing a row.
///
/// # Arguments
///
/// * `json_data` - A reference to a `serde_json::Value` containing the JSON data.
///
/// # Returns
///
/// * `(Vec<String>, Vec<Vec<Value>>)` - A tuple where the first element is a vector of headers
///   and the second element is a vector of rows. Each row is represented as a vector of `serde_json::Value`.
///
/// # Panics
///
/// This function will panic if:
///
/// * The JSON data does not contain a "resultSets" key.
/// * The "resultSets" key does not map to an array.
/// * The "headers" key does not map to an array of strings.
/// * The "rowSet" key does not map to an array of arrays.
///
/// # Examples
///
/// ```
/// use serde_json::Value;
///
/// let json_data: Value = serde_json::from_str(r#"
/// {
///     "resultSets": [
///         {
///             "headers": ["header1", "header2"],
///             "rowSet": [
///                 ["value1", "value2"],
///                 ["value3", "value4"]
///             ]
///         }
///     ]
/// }
/// "#).unwrap();
///
/// let (headers, rows) = extract_headers_and_rows(&json_data);
///
/// assert_eq!(headers, vec!["header1".to_string(), "header2".to_string()]);
/// assert_eq!(rows, vec![
///     vec![Value::String("value1".to_string()), Value::String("value2".to_string())],
///     vec![Value::String("value3".to_string()), Value::String("value4".to_string())]
/// ]);
/// ```
fn extract_headers_and_rows(json_data: &Value) -> (Vec<String>, Vec<Vec<Value>>) {
    let result_sets = json_data.as_object().unwrap().get("resultSets").unwrap();
    let headers = result_sets[0].get("headers").unwrap();
    let rows = result_sets[0].get("rowSet").unwrap();

    let headers: Vec<String> = headers
        .as_array()
        .unwrap()
        .iter()
        .map(|header| header.as_str().unwrap().to_string())
        .collect();
    let rows: Vec<Vec<Value>> = rows
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row.as_array().unwrap().clone())
        .collect();

    (headers, rows)
}

// Function to convert headers and rows into a Vec<Series>
fn json_to_series(headers: Vec<String>, rows: Vec<Vec<Value>>) -> Vec<Series> {
    headers
        .into_iter()
        .enumerate()
        .map(|(i, header)| {
            let column_data: Vec<AnyValue> = rows
                .iter()
                .map(|row| {
                    match &row[i] {
                        Value::Number(n) => AnyValue::Int32(n.as_i64().unwrap_or(0) as i32),
                        Value::String(s) => AnyValue::String(s),
                        _ => AnyValue::Null,
                    }
                })
                .collect();
            Series::new(&header, column_data)
        })
        .collect()
}

// Function to create a DataFrame from Vec<Series>
fn create_dataframe(series: Vec<Series>) -> DataFrame {
    DataFrame::new(series).expect("DataFrame creation should succeed")
}

// Function to write the DataFrame to a JSON file
fn write_dataframe_to_json(df: &mut DataFrame, path: &str) {
    let mut file = File::create(path).expect("file should create successfully");
    JsonWriter::new(&mut file)
        .with_json_format(JsonFormat::Json)
        .finish(df)
        .expect("JSON writing should succeed");
}

pub fn read_json_to_df(input_path: &str, output_path: &str) {
    let json_data = read_json_file(input_path);
    let (headers, rows) = extract_headers_and_rows(&json_data.unwrap());
    let series = json_to_series(headers, rows);
    let mut df = create_dataframe(series);
    write_dataframe_to_json(&mut df, output_path);
}

#[test]
pub fn json_data_test() {
    let input_path = "../data/json_data/test_output.json";
    let output_path = "../data/json_data/for_preprocessing_test_output.json";

    let json_data = read_json_file(input_path);
    let (headers, rows) = extract_headers_and_rows(&json_data.unwrap());
    let series = json_to_series(headers, rows);
    let mut df = create_dataframe(series);

    println!("{:?}", df);

    write_dataframe_to_json(&mut df, output_path);
}
