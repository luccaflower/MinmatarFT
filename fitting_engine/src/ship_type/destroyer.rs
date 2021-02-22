use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DestroyerType {
    T1,
    Interdictor,
    Command,
    Tactical,
}
