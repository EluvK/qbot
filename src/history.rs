use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::Path,
};

use serde::{Deserialize, Serialize};

/// deny lists for every group
#[derive(Default, Debug, Serialize, Deserialize)]
struct DenyLists(HashMap<String, HashSet<u64>>);

/// record running history for the bot, so the next run will automatically load these settings.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BotHistory {
    deny_lists: DenyLists,
}

static HISTORY_JSON_FILE: &str = "history.json";

pub enum BotHistoryOp {
    AddDeny(u64),
    RemoveDeny(u64),
}

impl BotHistory {
    /// constructor
    pub fn load() -> Self {
        let path = Path::new(HISTORY_JSON_FILE);

        if !path.exists() {
            let new_empty_history = Self::default();
            let mut _file = File::create(path).expect("Failed to create history file");
            new_empty_history.write_to_file();
            new_empty_history
        } else {
            Self::read_from_file()
        }
    }

    pub fn load_group_deny_history(&self) -> HashMap<u64, HashSet<u64>> {
        self.deny_lists
            .0
            .clone()
            .into_iter()
            .map(|(group_id_str, set)| (group_id_str.parse::<u64>().expect("error from string to u64"), set))
            .collect()
    }

    pub fn sync_op(&mut self, group_id: Option<u64>, op: BotHistoryOp) {
        let group_id_str = group_id.map_or(String::from("private"), |group_id| group_id.to_string());
        match op {
            BotHistoryOp::AddDeny(user_id) => {
                self.deny_lists
                    .0
                    .entry(group_id_str)
                    .or_insert(HashSet::new())
                    .insert(user_id);
            }
            BotHistoryOp::RemoveDeny(user_id) => {
                self.deny_lists
                    .0
                    .entry(group_id_str)
                    .or_insert(HashSet::new())
                    .remove(&user_id);
            }
        };
        self.write_to_file();
    }

    fn write_to_file(&self) {
        println!("[history] write_to_file: {:?}", self.deny_lists);
        let contents = serde_json::to_string_pretty(&self).expect("History ser error");
        std::fs::write(HISTORY_JSON_FILE, contents).expect("History file write error");
    }
    fn read_from_file() -> Self {
        println!("[history] read_from_file");
        let contents = std::fs::read(HISTORY_JSON_FILE).expect("History file read error");
        serde_json::from_slice::<Self>(&contents).expect("History de error")
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_bot_history_serde() {
        let raw_str = r#"
        {
            "deny_lists": {
                "123": [
                    1,
                    222,
                    3
                ]
            }
        }
        "#;
        let history = serde_json::from_str::<BotHistory>(raw_str).unwrap();
        println!("{:?}", history);
    }
}
