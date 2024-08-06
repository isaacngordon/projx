use derive_more::From;

pub type Result<T> = core::result::Result<T, LLMProviderError>;

#[derive(Debug, From)]
pub enum LLMProviderError {
    OllamaError(ollama_rs::error::OllamaError),
    
    #[from]
    Reqwest(reqwest::Error),
    #[from]
    SerdeJson(serde_json::Error),
    #[from]
    Tokio(tokio::task::JoinError),
    
    Other(String),
}