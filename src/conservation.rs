use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub struct Conversation {
    history: Vec<Message>,
}

impl Conversation {
    pub fn new(system_prompt: &str) -> Self {
        Self {
            history: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                }
            ],
        }
    }

    pub fn add_user(&mut self, user_input: &str) {
        self.history.push(Message {
            role: "user".to_string(),
            content: user_input.to_string(),
        });
    }

    pub fn add_assistant(&mut self, raw_text: String) {
        self.history.push(Message { role: "assistant".to_string(), content: raw_text });
    }

    pub fn history(&self) -> &[Message] {
        &self.history
    }
}
