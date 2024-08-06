use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use super::{error, LLM};

pub struct OllamaLLM; // Ensure this line is present

impl LLM for OllamaLLM {
    async fn prompt(&self, input: &str) -> super::error::Result<String> {
        // By default it will connect to localhost:11434
        let ollama = Ollama::default();

        // For custom values:
        // let ollama = Ollama::new("http://localhost".to_string(), 11434);

        let model = "llama2:latest".to_string();
        let prompt = input.to_string();

        let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

        match res {
            Ok(res) => Ok(res.response),
            Err(e) => Err(error::LLMProviderError::OllamaError(e))
        }
    }
}
