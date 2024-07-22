use super::LLM;

pub struct OllamaLLM;

impl LLM for OllamaLLM {
    fn prompt(&self, input: &str) -> String {
        // Use the ollama-rs crate to prompt the model
        let response = ollama_rs::prompt(input).await.unwrap_or_else(|_| "Error prompting Ollama".to_string());
        response
    }
}
