use fitting_engine::ship_type::battlecruiser::BattlecruiserType;
use fitting_engine::ship_type::battleship::BattleshipType;
use fitting_engine::ship_type::cruiser::CruiserType;
use fitting_engine::ship_type::frigate::FrigateType;
use fitting_engine::ship_type::ShipType;

pub fn ship_type_by_id(id: u64) -> ShipType {
    match id {
        25 => ShipType::Frigate(FrigateType::T1),
        831 => ShipType::Frigate(FrigateType::Interceptor),
        324 => ShipType::Frigate(FrigateType::Assault),
        1527 => ShipType::Frigate(FrigateType::Logistics),

        26 => ShipType::Cruiser(CruiserType::T1),
        358 => ShipType::Cruiser(CruiserType::HeavyAssault),
        894 => ShipType::Cruiser(CruiserType::HeavyInterdictor),
        832 => ShipType::Cruiser(CruiserType::Logistics),
        963 => ShipType::Cruiser(CruiserType::Strategic),
        833 => ShipType::Cruiser(CruiserType::ForceRecon),
        906 => ShipType::Cruiser(CruiserType::CombatRecon),

        27 => ShipType::Battleship(BattleshipType::T1),
        900 => ShipType::Battleship(BattleshipType::Marauder),

        419 => ShipType::Battlecruiser(BattlecruiserType::T1),
        1201 => ShipType::Battlecruiser(BattlecruiserType::T1),
        540 => ShipType::Battlecruiser(BattlecruiserType::Command),

        _ => panic!("{} inst a ship type id"),
    }
}
