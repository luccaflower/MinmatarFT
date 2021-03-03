use fitting_engine::faction::Faction;
use fitting_engine::fit::CompressedFit;
use fitting_engine::ship::RigSize;
use fitting_engine::ship::SensorStrengthType;
use fitting_engine::ship::Ship;
use fitting_engine::ship_type::battlecruiser::BattlecruiserType;
use fitting_engine::ship_type::battleship::BattleshipType;
use fitting_engine::ship_type::cruiser::CruiserType;
use fitting_engine::ship_type::destroyer::DestroyerType;
use fitting_engine::ship_type::frigate::FrigateType;
use fitting_engine::ship_type::ShipType;
use fitting_engine::stats::capacitor::Capacitor;
use fitting_engine::stats::defense::Defense;
use fitting_engine::stats::drone::Drone;
use fitting_engine::stats::fitting::Fitting;
use fitting_engine::stats::movement::Movement;
use fitting_engine::stats::sensor::Sensor;
use ts_rs::export;

export! {
    Faction => "bindings/Faction.ts",
    CompressedFit => "bindings/CompressedFit.ts",
    Ship => "bindings/Ship.ts",
    SensorStrengthType => "bindings/SensorStrengthType.ts",
    RigSize => "bindings/RigSize.ts",
    ShipType => "bindings/ShipType.ts",
    Fitting => "bindings/Fitting.ts",
    Defense => "bindings/Defense.ts",
    Movement => "bindings/Movement.ts",
    Sensor => "bindings/Sensor.ts",
    Drone => "bindings/Drone.ts",
    Capacitor => "bindings/Capacitor.ts",

    FrigateType => "bindings/FrigateType.ts",
    DestroyerType => "bindings/DestroyerType.ts",
    CruiserType => "bindings/CruiserType.ts",
    BattlecruiserType => "bindings/BattlecruiserType.ts",
    BattleshipType => "bindings/BattleshipType.ts",
}
