mod openai;
mod ollama;


pub trait LLM {
    async fn prompt(&self, input: &str) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::openai::{OpenAILLM};
    use super::ollama::{OllamaLLM};
    use tokio; // Ensure you have the tokio runtime for async tests

    #[tokio::test]
    async fn test_openai_llm() {
        let model = OpenAILLM;
        let response = model.prompt("Who is Marc?").await;
        assert!(response.is_ok(), "Response: {:?}", response.unwrap_err());
    }

    #[tokio::test]
    async fn test_ollama_llm() {
        let model = OllamaLLM;
        let response = model.prompt("Who is Marc?").await;
        assert!(response.is_ok(), "Response: {:?}", response.unwrap_err());
    }
}

