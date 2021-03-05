use std::str::FromStr;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum CruiserType {
    T1,
    HeavyAssault,
    HeavyInterdictor,
    Logistics,
    Strategic,
    ForceRecon,
    CombatRecon,
}
impl ToString for CruiserType {
    fn to_string(&self) -> String {
        match self {
            Self::T1 => "T1",
            Self::HeavyAssault => "HeavyAssault",
            Self::HeavyInterdictor => "HeavyInterdictor",
            Self::Logistics => "Logistics",
            Self::Strategic => "Strategic",
            Self::ForceRecon => "ForceRecon",
            Self::CombatRecon => "CombatRecon",
        }
        .to_string()
    }
}

impl FromStr for CruiserType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T1" => Ok(Self::T1),
            "HeavyAssault" => Ok(Self::HeavyAssault),
            "HeavyInterdictor" => Ok(Self::HeavyInterdictor),
            "Logistics" => Ok(Self::Logistics),
            "Strategic" => Ok(Self::Strategic),
            "ForceRecon" => Ok(Self::ForceRecon),
            "CombatRecon" => Ok(Self::CombatRecon),
            _ => Err(()),
        }
    }
}
