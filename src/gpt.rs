use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::role::ASSISTANT_SYSTEM_INFO;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum GPTRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GPTMessages {
    role: GPTRole,
    content: String,
}

impl GPTMessages {
    pub fn into_contnet(self) -> String {
        self.content
    }
    pub fn new_system_message(content: String) -> Self {
        Self {
            role: GPTRole::System,
            content,
        }
    }

    pub fn new_user_message(content: String) -> Self {
        Self {
            role: GPTRole::User,
            content,
        }
    }

    #[allow(dead_code)]
    pub fn new_assist_message(content: String) -> Self {
        Self {
            role: GPTRole::Assistant,
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GPTPostMessage {
    model: String,
    messages: Vec<GPTMessages>,
    max_tokens: u64,
    temperature: f64,
}

impl Default for GPTPostMessage {
    fn default() -> Self {
        Self {
            model: String::from("gpt-3.5-turbo"),
            messages: Vec::new(),
            max_tokens: 1000,
            temperature: 0.1,
        }
    }
}

impl GPTPostMessage {
    #[allow(dead_code)]
    pub fn new_basic_message(message: String) -> Self {
        GPTPostMessage {
            messages: vec![
                GPTMessages::new_system_message(ASSISTANT_SYSTEM_INFO.into()),
                GPTMessages::new_user_message(message),
            ],
            ..Default::default()
        }
    }

    pub fn new_with_history(histories: Vec<GPTMessages>) -> Self {
        GPTPostMessage {
            messages: histories,
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GPTUsage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}

impl Display for GPTUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Prompt tokens used: {}, Completion tokens used: {}, Total tokens used: {}",
            self.prompt_tokens, self.completion_tokens, self.total_tokens
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GPTRecvChoice {
    message: GPTMessages,

    // https://platform.openai.com/docs/guides/chat/response-format
    #[serde(skip)]
    _finish_reason: String, // not important actually, and it will be null but not "null", strange...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GPTRecvMessage {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<GPTRecvChoice>,
    usage: GPTUsage,
}

impl GPTRecvMessage {
    pub fn get_return_msg(&self) -> Option<GPTMessages> {
        println!("[COST] finish chatgpt-api call {}", self.usage);
        if self.choices.is_empty() {
            None
        } else {
            Some(self.choices[0].message.clone())
        }
    }
}
