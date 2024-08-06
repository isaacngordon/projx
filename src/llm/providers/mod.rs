mod error;
mod ollama;
mod openai;

pub use ollama::*;
pub use openai::*;

pub trait LLM {
    async fn prompt(&self, input: &str) -> error::Result<String>;
}

#[cfg(test)]
mod tests {
    use super::ollama::OllamaLLM;
    use super::openai::OpenAILLM;
    use super::*;
    use tokio; // Ensure you have the tokio runtime for async tests

    #[tokio::test]
    async fn test_openai_llm() {
        let model = OpenAILLM;
        let prompt = "Who is Marc?";
        let response = model.prompt(prompt).await;
        assert!(response.is_ok(), "Response: {:?}", response.unwrap_err());
        println!("Test Prompt: {}\nOpenAI: {:?}", prompt, response.unwrap());
    }

    #[tokio::test]
    async fn test_ollama_llm() {
        let model = OllamaLLM;
        let response = model.prompt("Who is Marc?").await;
        assert!(response.is_ok(), "Response: {:?}", response.unwrap_err());
        println!("Test Prompt: {}\nOllama: {:?}", "Who is Marc?", response.unwrap());
    }
}
