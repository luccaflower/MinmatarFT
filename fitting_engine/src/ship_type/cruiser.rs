use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CruiserType {
    T1,
    HeavyAssault,
    HeavyInterdictor,
    Logistics,
    Strategic,
    ForceRecon,
    CombatRecon,
}
