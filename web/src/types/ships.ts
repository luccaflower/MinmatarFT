export interface Ship {
    name: string;
    ship_type: ShipType;
    faction: Faction;
    high_slots: number;
    med_slots: number;
    low_slots: number;
    turret_hard_points: number;
    launcher_hard_points: number;
    rig_slots: number;
    rig_size: string;
    sensor_strength_type: string;
    fitting_stats: FittingStats;
    defensive_stats: DefenseStats;
    movement_stats: MovementStats;
    sensor_stats: SensorStats;
    drone_stats: DroneStats;
    capacitor_stats: CapacitorStats;
}

export interface CapacitorStats {
    capacitor_amount: number;
    capacitor_recharge_time: number;
    neut_resistance: number;
}

export interface DroneStats {
    control_range: number;
    capacity: number;
    bandwidth: number;
    max_drones: number;
}

export interface SensorStats {
    targeting_range: number;
    scan_res: number;
    sensor_strength: number;
    max_locked_targets: number;
}

export interface MovementStats {
    max_velocity: number;
    agility: number;
    mass: number;
    warp_speed: number;
}

export interface DefenseStats {
    hull_hp: number;
    hull_em_resists: number;
    hull_therm_resists: number;
    hull_kin_resists: number;
    hull_exp_resists: number;
    armor_hp: number;
    armor_em_resists: number;
    armor_therm_resists: number;
    armor_kin_resists: number;
    armor_exp_resists: number;
    shield_hp: number;
    shield_em_resists: number;
    shield_therm_resists: number;
    shield_kin_resists: number;
    shield_exp_resists: number;
    sig_radius: number;
}

export interface FittingStats {
    cpu: number;
    pg: number;
    calibration: number;
    cargo: number;
}

export type Faction =
    "Amarr"
    | "Minmatar"
    | "Caldari"
    | "Gallente"
    | "AngelCartel"
    | "MordusLegion"
    | "Guristas"
    | "BloodRaiders"
    | "SanshasNation"
    | "Serpentis"
    | "Triglavians"
    | "SistersOfEve"
    | "SocietyofConsciousThought"
    | "EdenCom"
    | "Concord"
    | "ORE"

export type ShipTypeKey = "Cruiser" | "Frigate" | "Battleship" | "Battlecruiser" | "Destroyer"

export type ShipTypeValue =
    "T1"
    | "Command"
    | "Marauder"
    | "HeavyAssault"
    | "HeavyInterdictor"
    | "Logistics"
    | "Strategic"
    | "ForceRecon"
    | "CombatRecon"
    | "Tactical"
    | "Interceptor" | "Assault" | "ElectronicAttack"

export type ShipType = { [key in ShipTypeKey]: ShipTypeValue }
