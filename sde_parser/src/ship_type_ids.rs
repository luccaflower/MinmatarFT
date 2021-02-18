use fitting_engine::ship_type::battlecruiser::BattlecruiserType;
use fitting_engine::ship_type::battleship::BattleshipType;
use fitting_engine::ship_type::cruiser::CruiserType;
use fitting_engine::ship_type::frigate::FrigateType;
use fitting_engine::ship_type::ShipType;

pub fn ship_type_by_id(id: u64) -> Option<ShipType> {
    match id {
        25 => Some(ShipType::Frigate(FrigateType::T1)),
        831 => Some(ShipType::Frigate(FrigateType::Interceptor)),
        324 => Some(ShipType::Frigate(FrigateType::Assault)),
        1527 => Some(ShipType::Frigate(FrigateType::Logistics)),

        26 => Some(ShipType::Cruiser(CruiserType::T1)),
        358 => Some(ShipType::Cruiser(CruiserType::HeavyAssault)),
        894 => Some(ShipType::Cruiser(CruiserType::HeavyInterdictor)),
        832 => Some(ShipType::Cruiser(CruiserType::Logistics)),
        963 => Some(ShipType::Cruiser(CruiserType::Strategic)),
        833 => Some(ShipType::Cruiser(CruiserType::ForceRecon)),
        906 => Some(ShipType::Cruiser(CruiserType::CombatRecon)),

        27 => Some(ShipType::Battleship(BattleshipType::T1)),
        900 => Some(ShipType::Battleship(BattleshipType::Marauder)),

        419 => Some(ShipType::Battlecruiser(BattlecruiserType::T1)),
        1201 => Some(ShipType::Battlecruiser(BattlecruiserType::T1)),
        540 => Some(ShipType::Battlecruiser(BattlecruiserType::Command)),

        _ => None,
    }
}
