use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    GraphQL {
        url: String,
        status_code: i32,
        message: String
    },

    #[from]
    SerdeJson(serde_json::Error)
}

// end boilerplate
