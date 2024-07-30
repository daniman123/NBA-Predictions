use data_fetching::utils::{
    process_bytes::decompress_bytes_to_string,
    read_write_from_file_tools::read_bytes_from_file,
};

#[test]
fn test_stored_bytes_read_convert() {
    // Read bytes from file
    let path = "../data/raw_response_data/test_output.bin";
    let bytes = read_bytes_from_file(path);
    assert!(bytes.is_ok());

    // Convert stored bytes to string
    let binding = bytes.unwrap();
    let string_data_result = decompress_bytes_to_string(&binding);
    assert!(string_data_result.is_ok());
    let string_data = string_data_result.unwrap();
    assert_ne!(string_data, " ");
    let json_data_result = serde_json::from_str::<serde_json::Value>(&string_data);
    assert!(json_data_result.is_ok());
}
