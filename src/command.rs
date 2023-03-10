use crate::role::BotRole;

pub(super) static COMMAND_HELP_INFO: &str = r#"
    - `#role { name }` to set role
        - assistant | taler | storyteller | default
    - `#clean` to clean chat history
    - `#reset` to reset everything
    - `#help` for this menu
    "#;

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
            "role" | "Role" => {
                if let Some(ins) = iter.next() {
                    Opcode::Role(BotRole::new_from_str(ins))
                } else {
                    // default is Assistant
                    Opcode::Role(BotRole::Assistant)
                }
            }
            "help" | "Help" => Opcode::Help,
            _ => Opcode::Invalid,
        }
    } else {
        Opcode::Invalid
    }
}
