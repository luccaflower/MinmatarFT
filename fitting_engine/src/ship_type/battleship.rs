use std::str::FromStr;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum BattleshipType {
    T1,
    Marauder,
}
impl ToString for BattleshipType {
    fn to_string(&self) -> String {
        match self {
            Self::T1 => "T1",
            Self::Marauder => "Marauder",
        }
        .to_string()
    }
}

impl FromStr for BattleshipType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T1" => Ok(Self::T1),
            "Marauder" => Ok(Self::Marauder),
            _ => Err(()),
        }
    }
}
