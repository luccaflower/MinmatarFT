use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrigateType {
    T1,
    Interceptor,
    Assault,
    ElectronicAttack,
    Logistics,
}
