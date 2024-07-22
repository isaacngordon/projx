pub struct OllamaLLM;

impl LLM for OllamaLLM {
    fn prompt(&self, input: &str) -> String {
        // Assuming the ollama-rs crate is already set up
        let response = ollama_rs::prompt(input).await.unwrap();
        response
    }
}
