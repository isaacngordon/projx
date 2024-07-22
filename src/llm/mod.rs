pub trait LLM {
    fn prompt(&self, input: &str) -> String;
}
