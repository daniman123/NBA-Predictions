pub mod matchups_dataframe;
mod round_3_processing;
mod round_4_processing;

use round_3_processing::round_3_processing_dataframe;
use round_4_processing::round_4_processing_dataframe;

fn main() {
    round_3_processing_dataframe();
    round_4_processing_dataframe();
}
