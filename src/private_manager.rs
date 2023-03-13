use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    command,
    config::Config,
    gpt::GPTMessages,
    history::{BotHistory, BotHistoryOp},
    role::BotRole,
};

use self::error::PrivateManagerError;

struct ChatContext {
    role: BotRole,
    _user_id: u64,
    wait_gpt_reply: bool, // to make sure one question at a time, to make it linear
    histories: Vec<GPTMessages>,
    last_ts: u64,
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
            last_ts: 0,
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

    fn query_too_often(&mut self, interval: u64, ts: u64) -> (bool, u64) {
        if self.last_ts + interval <= ts {
            if self.last_ts != 0 && self.last_ts + 60 * 60 * 2 <= ts {
                // 2 hours auto reset memory
                self.reset();
            }
            self.last_ts = ts;
            (false, 0)
        } else {
            (true, self.last_ts + interval - ts)
        }
    }
}

/// A PrivateManager handle one group / all private friend's conversations.
struct PrivateManager {
    contexts: HashMap<u64, ChatContext>,
    interval: u64,
    deny_list: HashSet<u64>,
}

impl PrivateManager {
    fn new(interval: u64) -> Self {
        PrivateManager {
            contexts: HashMap::new(),
            interval,
            deny_list: HashSet::new(),
        }
    }
    fn new_with_history(interval: u64, deny_list: HashSet<u64>) -> Self {
        PrivateManager {
            contexts: HashMap::new(),
            interval,
            deny_list,
        }
    }

    fn deny(&mut self, user_id: u64) {
        self.deny_list.insert(user_id);
    }

    fn allow(&mut self, user_id: u64) {
        self.deny_list.remove(&user_id);
    }

    fn is_deny(&self, qq: u64) -> bool {
        self.deny_list.contains(&qq)
    }

    fn reset(&mut self, user_id: Option<u64>) {
        match user_id {
            Some(user_id) => {
                self.contexts.get_mut(&user_id).unwrap().reset();
            }
            None => {
                // reset all
                self.contexts.iter_mut().for_each(|(_, c)| c.reset());
            }
        }
    }

    fn get_histories(&self, user_id: u64) -> Option<&Vec<GPTMessages>> {
        self.contexts.get(&user_id).map(|c| &c.histories)
    }

    fn push_history(&mut self, user_id: u64, msg: GPTMessages) {
        let chat_context = self.contexts.get_mut(&user_id).unwrap();
        chat_context.histories.push(msg);
    }

    fn pop_history(&mut self, user_id: u64) {
        let chat_context = self.contexts.get_mut(&user_id).unwrap();
        chat_context.histories.pop();
    }

    fn new_empty_context(&mut self, user_id: u64) {
        self.contexts
            .entry(user_id)
            .or_insert_with(|| ChatContext::new_chat_context(user_id));
    }

    /// when return Err(error), return to user immediately.
    /// when Ok(true), continue generate chatgpt answer.
    /// Noted: instructions should be the Err(), will not call chatgpt.
    fn pre_handle_private_message(
        &mut self,
        user_id: u64,
        ts: u64,
        message: String,
    ) -> Result<bool, PrivateManagerError> {
        if self.is_deny(user_id) {
            return Ok(false);
        }
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
            let (f, _t) = chat_context.query_too_often(self.interval, ts);
            if f {
                return Err(PrivateManagerError::QueryTooOften(_t));
            }
            chat_context.histories.push(GPTMessages::new_user_message(message));
            chat_context.wait_gpt_reply = true;

            Ok(true)
        }
    }

    fn after_handle_private_message(&mut self, user_id: u64) {
        let chat_context = self.contexts.get_mut(&user_id).unwrap();
        chat_context.wait_gpt_reply = false;
    }
}

pub struct ChatManager {
    group_contexts: HashMap<u64, PrivateManager>,
    private_context: PrivateManager,
    config: Rc<Config>,
    history: BotHistory,
}

impl ChatManager {
    pub fn new(config: Rc<Config>) -> Self {
        let history = BotHistory::load();
        let group_historys = history.load_group_deny_history();
        Self {
            group_contexts: group_historys
                .into_iter()
                .map(|(group_id, group_deny_history)| {
                    (group_id, PrivateManager::new_with_history(10, group_deny_history))
                })
                .collect(),
            private_context: PrivateManager::new(1),
            config,
            history,
        }
    }

    fn choose_pm(&mut self, group_id: Option<u64>) -> &mut PrivateManager {
        group_id.map_or(&mut self.private_context, |group_id| {
            self.group_contexts
                .entry(group_id)
                .or_insert_with(|| PrivateManager::new(10));
            self.group_contexts.get_mut(&group_id).unwrap()
        })
    }

    pub fn pre_handle_private_message(
        &mut self,
        group_id: Option<u64>,
        user_id: u64,
        ts: u64,
        message: String,
    ) -> Result<bool, PrivateManagerError> {
        self.choose_pm(group_id).new_empty_context(user_id); // just incase use sudo without any context.
        if let Some(root_instructions) = message.strip_prefix("#sudo") {
            if user_id != self.config.root_qq() {
                return Err(PrivateManagerError::PermissionDeny);
            }
            let err = match command::parse_root_instructions(root_instructions) {
                command::RootOpcode::Invalid => PrivateManagerError::InvalidCommand,
                command::RootOpcode::Deny(qq) => {
                    self.choose_pm(group_id).deny(qq);
                    self.history.sync_op(group_id, BotHistoryOp::AddDeny(qq));
                    PrivateManagerError::CommandSuccess(format!("Deny {}", qq))
                }
                command::RootOpcode::Allow(qq) => {
                    self.choose_pm(group_id).allow(qq);
                    self.history.sync_op(group_id, BotHistoryOp::RemoveDeny(qq));
                    PrivateManagerError::CommandSuccess(format!("Allow {}", qq))
                }
                command::RootOpcode::Reset(qq) => {
                    self.choose_pm(group_id).reset(Some(qq));
                    PrivateManagerError::CommandSuccess(format!("Reset {}", qq))
                }
                command::RootOpcode::ResetAll => {
                    self.choose_pm(group_id).reset(None);
                    PrivateManagerError::CommandSuccess("Reset All".into())
                }
            };
            return Err(err);
        }
        self.choose_pm(group_id)
            .pre_handle_private_message(user_id, ts, message)
    }
    pub fn after_handle_private_message(&mut self, group_id: Option<u64>, user_id: u64) {
        self.choose_pm(group_id).after_handle_private_message(user_id)
    }

    pub fn get_histories(&mut self, group_id: Option<u64>, user_id: u64) -> Option<&Vec<GPTMessages>> {
        self.choose_pm(group_id).get_histories(user_id)
    }

    pub fn push_history(&mut self, group_id: Option<u64>, user_id: u64, msg: GPTMessages) {
        self.choose_pm(group_id).push_history(user_id, msg)
    }

    pub fn pop_history(&mut self, group_id: Option<u64>, user_id: u64) {
        self.choose_pm(group_id).pop_history(user_id)
    }
}

mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum PrivateManagerError {
        #[error("Error: one message a time please.")]
        OnceMessageATime,

        #[error("Error: query too frequently. Wait {0} second please")]
        QueryTooOften(u64),

        #[error("Error: invalid command")]
        InvalidCommand,

        #[error("CommandSuccess: {0}")]
        CommandSuccess(String),

        #[error("Permission deny")]
        PermissionDeny,
    }
}
