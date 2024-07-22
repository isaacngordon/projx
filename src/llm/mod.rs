#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::{OpenAILLM, OllamaLLM};
    use tokio; // Ensure you have the tokio runtime for async tests

    #[tokio::test]
    async fn test_openai_llm() {
        let model = OpenAILLM;
        let response = model.prompt("Who is Marc?").await;
        println!("OpenAI Response: {}", response);
        // You can add assertions here based on expected output
    }

    #[tokio::test]
    async fn test_ollama_llm() {
        let model = OllamaLLM;
        let response = model.prompt("Who is Marc?").await;
        println!("Ollama Response: {}", response);
        // You can add assertions here based on expected output
    }
}
pub trait LLM {
    async fn prompt(&self, input: &str) -> String;
}
