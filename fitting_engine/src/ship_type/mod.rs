pub mod battlecruiser;
pub mod battleship;
pub mod cruiser;
pub mod destroyer;
pub mod frigate;

use crate::ship_type::battlecruiser::BattlecruiserType;
use crate::ship_type::battleship::BattleshipType;
use crate::ship_type::cruiser::CruiserType;
use crate::ship_type::destroyer::DestroyerType;
use crate::ship_type::frigate::FrigateType;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shoulda::Shoulda;
use std::str::FromStr;
#[cfg(feature = "ts")]
use ts_rs::TS;

#[derive(Debug, Clone, Shoulda)]
#[cfg_attr(feature = "ts", derive(TS))]
pub enum ShipType {
    Frigate(FrigateType),
    Destroyer(DestroyerType),
    Cruiser(CruiserType),
    Battlecruiser(BattlecruiserType),
    Battleship(BattleshipType),
}

impl<'de> Deserialize<'de> for ShipType {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let mut vec = Vec::<String>::deserialize(deserializer)?;
        if vec.len() == 2 {
            let inner = vec.pop().unwrap();
            let outer = vec.pop().unwrap();
            Ok(match outer.as_str() {
                "Frigate" => Self::Frigate(
                    FrigateType::from_str(inner.as_str()).map_err(|_| {
                        serde::de::Error::custom("couldnt parse frigate type")
                    })?,
                ),
                "Destroyer" => Self::Destroyer(
                    DestroyerType::from_str(inner.as_str()).map_err(|_| {
                        serde::de::Error::custom("couldnt parse destroyer type")
                    })?,
                ),
                "Cruiser" => Self::Cruiser(
                    CruiserType::from_str(inner.as_str()).map_err(|_| {
                        serde::de::Error::custom("couldnt parse cruiser type")
                    })?,
                ),
                "Battlecruiser" => Self::Battlecruiser(
                    BattlecruiserType::from_str(inner.as_str()).map_err(
                        |_| {
                            serde::de::Error::custom(
                                "couldnt parse battlecruiser type",
                            )
                        },
                    )?,
                ),
                "Battleship" => Self::Battleship(
                    BattleshipType::from_str(inner.as_str()).map_err(|_| {
                        serde::de::Error::custom(
                            "couldnt parse battleship type",
                        )
                    })?,
                ),
                _ => Err(serde::de::Error::custom(
                    "couldnt parse outer shiptype",
                ))?,
            })
        } else {
            Err(serde::de::Error::custom("incorrect length of array"))
        }
    }
}

impl Serialize for ShipType {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            ShipType::Frigate(x) => ["Frigate".to_string(), x.to_string()],
            ShipType::Destroyer(x) => ["Destroyer".to_string(), x.to_string()],
            ShipType::Cruiser(x) => ["Cruiser".to_string(), x.to_string()],
            ShipType::Battlecruiser(x) => {
                ["Battlecruiser".to_string(), x.to_string()]
            }
            ShipType::Battleship(x) => {
                ["Battleship".to_string(), x.to_string()]
            }
        }
        .serialize(serializer)
    }
}
