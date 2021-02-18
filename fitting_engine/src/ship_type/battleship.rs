use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BattleshipType {
    T1,
    Marauder,
}
