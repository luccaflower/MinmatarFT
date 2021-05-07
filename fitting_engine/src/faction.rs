use assertable::Assertable;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, Assertable)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum Faction {
    Amarr,
    Minmatar,
    Caldari,
    Gallente,
    AngelCartel,
    MordusLegion,
    Guristas,
    BloodRaiders,
    SanshasNation,
    Serpentis,
    Triglavians,
    SistersOfEve,
    SocietyofConsciousThought,
    EdenCom,
    Concord,
    ORE,
}
