pub mod matchups_dataframe;
mod round_3_processing;
mod round_4_processing;
mod round_5_processing;

use round_3_processing::round_3_processing_dataframe;
use round_4_processing::round_4_processing_dataframe;
use round_5_processing::round_5_processing_dataframe;

fn main() {
    round_3_processing_dataframe();
    round_4_processing_dataframe();
    round_5_processing_dataframe();
}
