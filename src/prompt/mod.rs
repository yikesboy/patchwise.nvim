pub mod edit;

#[derive(Debug, Clone)]
pub struct Prompt {
    text: String,
}

impl Prompt {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.text
    }
}

impl AsRef<str> for Prompt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
