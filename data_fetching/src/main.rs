mod error;
pub mod decompressed_json_reader;
pub mod game_log_json_data_into_data_frame;
pub mod utils;
mod fetch_endpoint_response;
pub use self::error::{ Error, Result };

fn main() {
    println!("Hello World!");
}
