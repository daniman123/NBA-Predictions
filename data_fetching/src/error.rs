use derive_more::From;
use reqwest::header::InvalidHeaderValue;

pub type Result<T> = core::result::Result<T, Error>;
// pub type Error = Box<dyn std::error::Error>; // Early Dev Stage

#[derive(Debug, From)]
pub enum Error {
    #[from]
    InvalidHeader(InvalidHeaderValue),
}
