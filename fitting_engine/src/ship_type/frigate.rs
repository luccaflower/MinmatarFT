use assertable::Assertable;
use std::str::FromStr;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Assertable)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum FrigateType {
    T1,
    Interceptor,
    Assault,
    ElectronicAttack,
    Logistics,
}
impl ToString for FrigateType {
    fn to_string(&self) -> String {
        match self {
            Self::T1 => "T1",
            Self::Interceptor => "Interceptor",
            Self::Assault => "Assault",
            Self::ElectronicAttack => "ElectronicAttack",
            Self::Logistics => "Logistics",
        }
        .to_string()
    }
}

impl FromStr for FrigateType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T1" => Ok(Self::T1),
            "Interceptor" => Ok(Self::Interceptor),
            "Assault" => Ok(Self::Assault),
            "ElectronicAttack" => Ok(Self::ElectronicAttack),
            "Logistics" => Ok(Self::Logistics),
            _ => Err(()),
        }
    }
}
