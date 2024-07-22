pub struct OpenAILLM;

impl LLM for OpenAILLM {
    fn prompt(&self, input: &str) -> String {
        let client = reqwest::Client::new();
        let response = client.post("https://api.openai.com/v1/engines/davinci-codex/completions")
            .header("Authorization", "Bearer YOUR_API_KEY")
            .json(&serde_json::json!({
                "prompt": input,
                "max_tokens": 100,
            }))
            .send()
            .await
            .unwrap();

        let json: serde_json::Value = response.json().await.unwrap();
        json["choices"][0]["text"].as_str().unwrap_or("").to_string()
    }
}
