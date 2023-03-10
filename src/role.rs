use crate::gpt::GPTMessages;

// as default system info.
pub static ASSISTANT_SYSTEM_INFO :&str = "You are a helpful assistant. Unless there is a specific word count requirement, please answer the question as concisely as possible";
static STORYTELLER_SYSTEM_INFO :&str = "I want you to act as a storyteller. You will come up with entertaining stories that are engaging, imaginative and captivating for the audience. It can be fairy tales, educational stories or any other type of stories which has the potential to capture people's attention and imagination. Depending on the target audience, you may choose specific themes or topics for your storytelling session e.g., if it’s children then you can talk about animals; If it’s adults then history-based tales might engage them better etc. ";
static TALER_SYSTEM_INFO :&str = "I need you to act as an assistant to a fairy tale writer and help the writer write fairy tales for given keywords. The language must be familiar and easy to understand, the logic of the story must be smooth, the content of the story must be imaginative and interesting, and have certain educational significance .";

#[derive(Debug, Clone)]
pub enum BotRole {
    Assistant,
    Taler,
    StoryTeller,
}

impl BotRole {
    pub fn new_from_str(ins: &str) -> Self {
        match ins {
            "assistant" | "Assistant" | "default" | "Default" => Self::Assistant,
            "taler" => Self::Taler,
            "storyteller" => Self::StoryTeller,
            _ => Self::Assistant,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            BotRole::Assistant => {
                "I am now a Assistant, I will answer the question as concisely as possible unless work count specified"
            }
            BotRole::Taler => "I am now a fairy tale writer, I will help you write stories.",
            BotRole::StoryTeller => "I am now a story teller, I will help you wirte stories.",
        }
    }
    pub fn system_infomation(&self) -> GPTMessages {
        match self {
            BotRole::Assistant => GPTMessages::new_system_message(ASSISTANT_SYSTEM_INFO.into()),
            BotRole::Taler => GPTMessages::new_system_message(TALER_SYSTEM_INFO.into()),
            BotRole::StoryTeller => GPTMessages::new_system_message(STORYTELLER_SYSTEM_INFO.into()),
        }
    }
}
