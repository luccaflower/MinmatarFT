pub mod battlecruiser;
pub mod battleship;
pub mod cruiser;
pub mod destroyer;
pub mod frigate;
pub mod stat_modification;

use crate::ship_type::battlecruiser::BattlecruiserType;
use crate::ship_type::battleship::BattleshipType;
use crate::ship_type::cruiser::CruiserType;
use crate::ship_type::destroyer::DestroyerType;
use crate::ship_type::frigate::FrigateType;

#[derive(Debug, Clone)]
pub enum ShipType {
    Frigate(FrigateType),
    Destroyer(DestroyerType),
    Cruiser(CruiserType),
    Battlecruiser(BattlecruiserType),
    Battleship(BattleshipType),
}
