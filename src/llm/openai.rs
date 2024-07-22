use super::LLM;
use serde_json;

pub struct OpenAILLM;

impl LLM for OpenAILLM {
    async fn prompt(&self, input: &str) -> String {
        let openai_api_key = &std::env::var("OPENAI_API_KEY")
            .expect("OPENAI_API_KEY not set");
        
        let client = reqwest::Client::new();
        let response = client.post("https://api.openai.com/v1/engines/davinci-codex/completions")
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .json(&serde_json::json!({
                "prompt": input,
                "max_tokens": 100,
            }))
            .send()
            .await
            .unwrap();

        let json: serde_json::Value = response.json().await.unwrap();
        if (json["error"].is_object()) {
            panic!("Code: {} Error: {}", json["error"]["code"], json["error"]["message"]);
        }
        
        json["choices"][0]["text"].as_str().unwrap_or("").to_string()
    }
}
