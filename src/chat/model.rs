use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Model {
    pub model: LanguageModel,
    pub max_token: u32,
}

#[derive(Deserialize, Serialize)]
pub enum LanguageModel {
    OpenAiGpt3Turbo,
    DeepSeekChat,
    DeepSeekCode,
}

impl Default for Model {
    fn default() -> Self {
        Model {
            model: LanguageModel::OpenAiGpt3Turbo,
            max_token: 2048,
        }
    }
}

impl Model {
    pub fn api_url(&self) -> String {
        match self.model {
            LanguageModel::OpenAiGpt3Turbo => "https://api.openai.com/v1/chat/completions",
            LanguageModel::DeepSeekChat | LanguageModel::DeepSeekCode => "https://api.deepseek.com/chat/completions",
        }
        .into()
    }

    pub fn model_str(&self) -> &'static str {
        match self.model {
            LanguageModel::OpenAiGpt3Turbo => "gpt-3.5-turbo",
            LanguageModel::DeepSeekChat => "deepseek-chat",
            LanguageModel::DeepSeekCode => "deepseek-coder",
        }
    }
}
