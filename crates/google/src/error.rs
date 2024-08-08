use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    ReqwestError(reqwest::Error),
    #[from]
    IoError(std::io::Error),
    #[from]
    SerdeJsonError(serde_json::Error),

    CustomError(String),
}

// end boilerplate
