use super::LLM;
use serde_json;

pub struct OpenAILLM;

impl LLM for OpenAILLM {
    async fn prompt(&self, input: &str) -> Result<String, String> {
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
            .await
            .unwrap();

        let json: serde_json::Value = response.json().await.unwrap();
        if json["error"].is_object() {
            return Err(format!(
                "Code: {} Error: {}",
                json["error"]["code"], json["error"]["message"]
            ));
        }

        let ret = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        Ok(ret)
    }
}
