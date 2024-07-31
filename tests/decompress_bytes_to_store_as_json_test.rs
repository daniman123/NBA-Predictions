use data_fetching::{
    decompressed_json_reader::read_decompressed_json,
    utils::read_write_from_file_tools::write_json_to_file,
};

#[test]
fn read_bytes_store_json_test() {
    let path = "../data/raw_response_data/test_output.bin";
    let json_data_result = read_decompressed_json(path);
    assert!(json_data_result.is_ok());
    let json_data = json_data_result.unwrap();

    // Store json
    let path = "../data/json_data/test_output.json";
    let write_json_result = write_json_to_file(path, json_data);
    assert!(write_json_result.is_ok())
}
