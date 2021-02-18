use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CruiserType {
    T1,
    HeavyAssault,
    HeavyInterdictor,
    Logistics,
    Strategic,
    ForceRecon,
    CombatRecon,
}
