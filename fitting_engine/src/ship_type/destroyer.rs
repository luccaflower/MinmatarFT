use shoulda::Shoulda;
use std::str::FromStr;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Shoulda)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum DestroyerType {
    T1,
    Interdictor,
    Command,
    Tactical,
}
impl ToString for DestroyerType {
    fn to_string(&self) -> String {
        match self {
            Self::T1 => "T1",
            Self::Interdictor => "Interdictor",
            Self::Command => "Command",
            Self::Tactical => "Tactical",
        }
        .to_string()
    }
}

impl FromStr for DestroyerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "T1" => Ok(Self::T1),
            "Interdictor" => Ok(Self::Interdictor),
            "Command" => Ok(Self::Command),
            "Tactical" => Ok(Self::Tactical),
            _ => Err(()),
        }
    }
}
