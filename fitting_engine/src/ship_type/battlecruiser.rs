use shoulda::Shoulda;
use std::str::FromStr;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Shoulda)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum BattlecruiserType {
    T1,
    Command,
}

impl ToString for BattlecruiserType {
    fn to_string(&self) -> String {
        match self {
            Self::T1 => "T1",
            Self::Command => "Command",
        }
        .to_string()
    }
}

impl FromStr for BattlecruiserType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T1" => Ok(Self::T1),
            "Command" => Ok(Self::Command),
            _ => Err(()),
        }
    }
}
