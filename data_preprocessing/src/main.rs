pub mod error;
pub mod matchups_dataframe;
pub mod response_data_extractors;
mod round_2_processing;
mod round_3_processing;
mod round_4_processing;
mod round_5_processing;

pub use self::error::{Error, Result};
use round_2_processing::round_2_processing_dataframe;
use round_3_processing::round_3_processing_dataframe;
use round_4_processing::round_4_processing_dataframe;
use round_5_processing::round_5_processing_dataframe;

fn main() -> Result<()> {
    round_2_processing_dataframe()?;
    round_3_processing_dataframe();
    round_4_processing_dataframe();
    round_5_processing_dataframe();
    Ok(())
}
