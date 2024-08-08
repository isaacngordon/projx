use super::error;
use super::LLM;
use serde_json;

pub struct OpenAILLM;

impl LLM for OpenAILLM {
    async fn prompt(&self, input: &str) -> error::Result<String> {
        let openai_api_key = &std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", openai_api_key))
            .json(&serde_json::json!({
                "model": "gpt-4o",
                "max_tokens": 100,
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant."
                    },
                    {
                        "role": "user",
                        "content": input
                    }
                ]
            }))
            .send()
            .await?;

        let json: serde_json::Value = response.json().await?;

        if json["error"].is_object() {
            return Err(error::LLMProviderError::Other(format!(
                "Code: {} Error: {}",
                json["error"]["message"]
                    .as_str()
                    .unwrap_or("<no message>")
                    .to_string(),
                json["error"]["code"]
                    .as_str()
                    .unwrap_or("<no code>")
                    .to_string()
            )));
        }

        let ret = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| error::LLMProviderError::Other("<No content>".to_string()))?
            .to_string();

        Ok(ret)
    }
}
