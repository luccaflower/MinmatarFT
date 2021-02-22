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
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ShipType {
    Frigate(FrigateType),
    Destroyer(DestroyerType),
    Cruiser(CruiserType),
    Battlecruiser(BattlecruiserType),
    Battleship(BattleshipType),
}
