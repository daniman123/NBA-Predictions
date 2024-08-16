use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    CannotConvertStringToJson(serde_json::Error),
    #[from]
    FailedToWriteBytesToFile(std::io::Error),
}
