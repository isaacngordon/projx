use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    GraphQLCustomError {
        url: String,
        status_code: u16,
        message: String,
    },

    #[from]
    SerdeJson(serde_json::Error),

    #[from]
    ReqwestError(reqwest::Error),

    #[from]
    CynicGraphQlError(cynic::GraphQlError),

    CustomError(String),
}

// end boilerplate
