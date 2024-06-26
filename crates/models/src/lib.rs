use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadOnlyInstruction {
    pub program_id: String,
    pub data: String,
    pub accounts: Vec<String>,
    pub inner_instructions: Vec<ReadOnlyInstruction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadOnlyTransaction {
    pub signature: String,
    pub log_messages: Vec<String>,
    pub accounts: Vec<String>,
    pub instructions: Vec<ReadOnlyInstruction>,
}
