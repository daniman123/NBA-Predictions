use polars::prelude::*;

pub fn read_df_from_json(path: &str) -> DataFrame {
    let mut file = std::fs::File::open(path).unwrap();
    JsonReader::new(&mut file).finish().unwrap()
}

pub fn write_df_to_json(path: &str, mut df: DataFrame) {
    let mut file = std::fs::File::create(path).unwrap();
    // json
    JsonWriter::new(&mut file).with_json_format(JsonFormat::Json).finish(&mut df).unwrap();
}