use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ModelConfiguration {
    pub model: LanguageModel,
    pub max_token: u32,
}

#[derive(Deserialize, Serialize)]
pub enum LanguageModel {
    #[serde(rename = "gpt-3.5-turbo")]
    OpenAiGpt3Turbo,
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-coder")]
    DeepSeekCode,
}

impl Default for ModelConfiguration {
    fn default() -> Self {
        ModelConfiguration {
            model: LanguageModel::OpenAiGpt3Turbo,
            max_token: 2048,
        }
    }
}

impl ModelConfiguration {
    pub fn deepseek_chat() -> ModelConfiguration {
        ModelConfiguration {
            model: LanguageModel::DeepSeekChat,
            max_token: 2048,
        }
    }

    pub fn deepseek_code() -> ModelConfiguration {
        ModelConfiguration {
            model: LanguageModel::DeepSeekCode,
            max_token: 2048,
        }
    }

    pub fn openai() -> ModelConfiguration {
        ModelConfiguration {
            model: LanguageModel::OpenAiGpt3Turbo,
            max_token: 2048,
        }
    }

    pub fn api_url(&self) -> String {
        match self.model {
            LanguageModel::OpenAiGpt3Turbo => "https://api.openai.com/v1/chat/completions",
            LanguageModel::DeepSeekChat | LanguageModel::DeepSeekCode => "https://api.deepseek.com/chat/completions",
        }
        .into()
    }
}
