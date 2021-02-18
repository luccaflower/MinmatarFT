use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
