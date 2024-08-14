use derive_more::From;
use reqwest::header::InvalidHeaderValue;
use std::str::Utf8Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    InvalidHeader(InvalidHeaderValue),

    #[from]
    FetchRequestError(reqwest::Error),
    #[from]
    FailedToWriteBytesToFile(std::io::Error),
    #[from]
    CannotReadStringFromBytes(Utf8Error),
    #[from]
    CannotConvertStringToJson(serde_json::Error),
}
