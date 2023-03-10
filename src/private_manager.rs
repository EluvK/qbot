use std::collections::HashMap;

use crate::{command, gpt::GPTMessages, role::BotRole};

use self::error::PrivateManagerError;

struct ChatContext {
    role: BotRole,
    _user_id: u64,
    wait_gpt_reply: bool, // to make sure one question at a time, to make it linear
    histories: Vec<GPTMessages>,
    _max_depth: usize,
}

const DEFAULT_MAX_DEPTH: usize = 20; // seems useless. limited by 4096 tokon

impl ChatContext {
    fn new_chat_context(user_id: u64) -> Self {
        let mut new_context = ChatContext {
            role: BotRole::Assistant,
            _user_id: user_id,
            wait_gpt_reply: false,
            histories: Vec::new(),
            _max_depth: DEFAULT_MAX_DEPTH,
        };
        new_context.set_default_role();
        new_context
    }

    fn reset(&mut self) {
        self.histories.clear();
        self.set_default_role()
    }

    fn clean(&mut self) {
        let role = self.role.clone();
        self.set_role(&role);
    }

    fn set_default_role(&mut self) {
        self.set_role(&BotRole::Assistant)
    }

    fn set_role(&mut self, role: &BotRole) {
        self.histories.clear();
        self.role = role.clone();
        role.system_infomation()
            .iter()
            .for_each(|m| self.histories.push(m.clone()));
    }
}

/// manager each private friends conversations
pub struct PrivateManager {
    contexts: HashMap<u64, ChatContext>,
}

impl PrivateManager {
    pub fn new() -> Self {
        PrivateManager {
            contexts: HashMap::new(),
        }
    }

    pub fn get_histories(&self, user_id: u64) -> Option<&Vec<GPTMessages>> {
        self.contexts.get(&user_id).map(|c| &c.histories)
    }

    pub fn push_history(&mut self, user_id: u64, msg: GPTMessages) {
        let chat_context = self.contexts.get_mut(&user_id).unwrap();
        chat_context.histories.push(msg);
    }

    pub fn pop_history(&mut self, user_id: u64) {
        let chat_context = self.contexts.get_mut(&user_id).unwrap();
        chat_context.histories.pop();
    }

    /// when return error, return to user immediately.
    /// when ok, continue generate chatgpt answer.
    /// Noted: instructions should be the Err(), will not call chatgpt.
    pub fn pre_handle_private_message(&mut self, user_id: u64, message: String) -> Result<(), PrivateManagerError> {
        self.contexts
            .entry(user_id)
            .or_insert_with(|| ChatContext::new_chat_context(user_id));

        let chat_context = self.contexts.get_mut(&user_id).unwrap();

        // instructions:
        if let Some(instructions) = message.strip_prefix('#') {
            let err = match command::parse_instructions(instructions) {
                command::Opcode::Invalid => PrivateManagerError::InvalidCommand,
                command::Opcode::Role(r) => {
                    chat_context.set_role(&r);
                    PrivateManagerError::CommandSuccess(format!("> set role succ {}", r.description()))
                }
                command::Opcode::Clean => {
                    chat_context.clean();
                    PrivateManagerError::CommandSuccess("> clean success".into())
                }
                command::Opcode::Reset => {
                    chat_context.reset();
                    PrivateManagerError::CommandSuccess("> reset success".into())
                }
                command::Opcode::Help => {
                    PrivateManagerError::CommandSuccess(format!("> help:\n{}", command::COMMAND_HELP_INFO))
                }
            };
            Err(err)
        } else {
            if chat_context.wait_gpt_reply {
                return Err(PrivateManagerError::OnceMessageATime);
            }
            chat_context.histories.push(GPTMessages::new_user_message(message));
            chat_context.wait_gpt_reply = true;

            Ok(())
        }
    }

    pub fn after_handle_private_message(&mut self, user_id: u64) {
        let chat_context = self.contexts.get_mut(&user_id).unwrap();
        chat_context.wait_gpt_reply = false;
    }
}

pub struct GroupManager {
    contexts: HashMap<u64, PrivateManager>,
}

impl GroupManager {
    pub fn new() -> Self {
        Self {
            contexts: HashMap::new(),
        }
    }

    pub fn pre_handle_private_message(
        &mut self,
        group_id: u64,
        user_id: u64,
        message: String,
    ) -> Result<(), PrivateManagerError> {
        self.contexts.entry(group_id).or_insert_with(|| PrivateManager::new());
        let group_private_manager = self.contexts.get_mut(&group_id).unwrap();
        group_private_manager.pre_handle_private_message(user_id, message)
    }

    pub fn get_histories(&mut self, group_id: u64, user_id: u64) -> Option<&Vec<GPTMessages>> {
        self.contexts.get_mut(&group_id).unwrap().get_histories(user_id)
    }

    pub fn push_history(&mut self, group_id: u64, user_id: u64, msg: GPTMessages) {
        self.contexts.get_mut(&group_id).unwrap().push_history(user_id, msg)
    }

    pub fn pop_history(&mut self, group_id: u64, user_id: u64) {
        self.contexts.get_mut(&group_id).unwrap().pop_history(user_id)
    }

    pub fn after_handle_private_message(&mut self, group_id: u64, user_id: u64) {
        self.contexts
            .get_mut(&group_id)
            .unwrap()
            .after_handle_private_message(user_id)
    }
}

mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum PrivateManagerError {
        #[error("Error: one message a time please.")]
        OnceMessageATime,

        #[error("Error: invalid command")]
        InvalidCommand,

        #[error("CommandSuccess: {0}")]
        CommandSuccess(String),
    }
}
