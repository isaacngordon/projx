pub trait LLM {
    async fn prompt(&self, input: &str) -> String;
}
