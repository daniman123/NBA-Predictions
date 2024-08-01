use polars::prelude::*;
use serde_json::Value;
use std::fs;

#[test]
pub fn json_data_test() {
    let path = "../data/json_data/test_output.json";
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
                .map(|row| {
                    match &row[i] {
                        Value::Number(n) => AnyValue::Int32(n.as_i64().unwrap_or(0) as i32),
                        Value::String(s) => AnyValue::String(s),
                        _ => AnyValue::Null,
                    }
                })
                .collect();
            Series::new(header.as_str().unwrap(), column_data)
        })
        .collect();

    // Create DataFrame
    let mut df = DataFrame::new(series).unwrap();

    // Print DataFrame
    println!("{:?}", df);

    let mut file = std::fs::File
        ::create("../data/json_data/for_preprocessing_test_output.json")
        .unwrap();

    JsonWriter::new(&mut file).with_json_format(JsonFormat::Json).finish(&mut df).unwrap();
}
