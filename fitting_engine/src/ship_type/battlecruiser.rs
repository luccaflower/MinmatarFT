use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattlecruiserType {
    T1,
    Command,
}
