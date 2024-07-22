use super::LLM;

pub struct OllamaLLM; // Ensure this line is present

impl LLM for OllamaLLM {
    async fn prompt(&self, input: &str) -> String { // Change this line to async
        // Use the ollama-rs crate to prompt the model
        let response = ollama_rs::prompt(input).await.unwrap_or_else(|_| "Error prompting Ollama".to_string());
        response
    }
}

impl LLM for OllamaLLM {
    fn prompt(&self, input: &str) -> String {
        // Use the ollama-rs crate to prompt the model
        let response = ollama_rs::prompt(input).await.unwrap_or_else(|_| "Error prompting Ollama".to_string());
        response
    }
}
