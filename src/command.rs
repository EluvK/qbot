use crate::role::BotRole;

pub(super) static COMMAND_HELP_INFO: &str = r#"
    - `#role { name }` to set role
        - assistant | clippy | taler | storyteller | foodcritic | default
    - `#clean` to clean chat history
    - `#reset` to reset everything
    - `#help` for this menu
    "#;

pub(super) static GREETING_MESSAGE: &str = "Hi! I'm a chatbot developed by EluvK and open-sourced at https://github.com/EluvK/qbot. Type `#help` to get the list of commands.";

pub(super) enum Opcode {
    Invalid,
    Role(BotRole),
    Reset,
    Clean,
    Help,
}

pub(super) fn parse_instructions(ins: &str) -> Opcode {
    let mut iter = ins.split_whitespace();
    if let Some(op) = iter.next() {
        match op {
            "clean" | "Clean" => Opcode::Clean,
            "reset" | "Reset" => Opcode::Reset,
            "role" | "Role" => Opcode::Role(iter.next().map_or(BotRole::Assistant, BotRole::new_from_str)),
            "help" | "Help" => Opcode::Help,
            _ => Opcode::Invalid,
        }
    } else {
        Opcode::Invalid
    }
}

pub(super) enum RootOpcode {
    Invalid,
    Deny(u64), // group ignore a user permanently
    Allow(u64),
    Reset(u64), // root reset someone's contexts
    ResetAll,   // root reset this group context all
}

pub(super) fn parse_root_instructions(ins: &str) -> RootOpcode {
    let mut iter = ins.split_whitespace();
    if let Some(root_op) = iter.next() {
        match (root_op, iter.next()) {
            ("deny" | "Deny", Some(ins)) => {
                if let Ok(qq_number) = ins.parse::<u64>() {
                    return RootOpcode::Deny(qq_number);
                }
            }
            ("allow" | "Allow", Some(ins)) => {
                if let Ok(qq_number) = ins.parse::<u64>() {
                    return RootOpcode::Allow(qq_number);
                }
            }
            ("reset" | "Reset", None) => {
                return RootOpcode::ResetAll;
            }
            ("reset" | "Reset", Some(ins)) => {
                if let Ok(qq_number) = ins.parse::<u64>() {
                    return RootOpcode::Reset(qq_number);
                }
            }

            _ => {}
        }
    }
    RootOpcode::Invalid
}
