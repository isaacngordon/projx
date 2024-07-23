use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

use super::LLM;

pub struct OllamaLLM; // Ensure this line is present

impl LLM for OllamaLLM {
    async fn prompt(&self, input: &str) -> Result<String, String> {
        // By default it will connect to localhost:11434
        let ollama = Ollama::default();

        // For custom values:
        // let ollama = Ollama::new("http://localhost".to_string(), 11434);

        let model = "llama2:latest".to_string();
        let prompt = input.to_string();

        let res = ollama.generate(
            GenerationRequest::new(model, prompt)
        ).await;

        let ret = if let Ok(res) = res {
            println!("{}", res.response);
            res.response
        } else {
            println!("Error: {:?}", res);
            "".to_string()
        };
        Ok(ret) // Return the response
    }
}
