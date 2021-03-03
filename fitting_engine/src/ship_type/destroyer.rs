use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum DestroyerType {
    T1,
    Interdictor,
    Command,
    Tactical,
}
