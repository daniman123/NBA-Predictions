use serde_json::{Value, Error};

/// Converts a JSON string into a serde_json::Value.
///
/// # Arguments
///
/// * `json_str` - A string slice that holds the JSON data.
///
/// # Returns
///
/// * `Result<Value, Error>` - A `Result` containing the JSON value if successful, or an error if the parsing fails.
///
/// # Errors
///
/// This function will return an error if the JSON string is not valid.
pub fn convert_string_to_json(json_str: &str) -> Result<Value, Error> {
    serde_json::from_str(json_str)
}

#[test]
fn test_convert_string_to_json() {
    
    let json_str = r#"{
        "name": "John Doe",
        "age": 30,
        "is_active": true
    }"#;

    match convert_string_to_json(json_str) {
        Ok(json_value) => {
            println!("Parsed JSON: {}", json_value);
        },
        Err(e) => {
            println!("Failed to parse JSON: {}", e);
        },
    }
}
