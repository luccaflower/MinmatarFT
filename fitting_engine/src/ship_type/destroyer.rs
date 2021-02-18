use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DestroyerType {
    T1,
    Interdictor,
    Command,
    Tactical,
}
