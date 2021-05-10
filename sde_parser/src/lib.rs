use crate::faction_ids::faction_by_id;
use crate::model::category_id::CategoryId;
use crate::model::dogma_attribute::DogmaAttribute;
use crate::model::group_id::GroupId;
use crate::model::type_dogma::TypeDogma;
use crate::model::type_id::TypeId;
use crate::ship_type_ids::ship_type_by_id;
use fitting_engine::faction::Faction;
use fitting_engine::fit::Modules;
use fitting_engine::ship::{RigSize, SensorStrengthType, Ship};
use fitting_engine::static_module::{FittingMod, ModuleSlot, StaticModule};
use fitting_engine::stats::capacitor::Capacitor;
use fitting_engine::stats::capacitor::CapacitorModifications;
use fitting_engine::stats::defense::Defense;
use fitting_engine::stats::defense::DefenseModifications;
use fitting_engine::stats::drone::Drone;
use fitting_engine::stats::drone::DroneModifications;
use fitting_engine::stats::fitting::Fitting;
use fitting_engine::stats::fitting::FittingModifications;
use fitting_engine::stats::movement::Movement;
use fitting_engine::stats::movement::MovementModifications;
use fitting_engine::stats::sensor::Sensor;
use fitting_engine::stats::sensor::SensorModifications;
use fitting_engine::stats::ModificationType;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Write;

mod extract_resistance;
mod faction_ids;
pub mod model;
mod ship_type_ids;

#[derive(Debug)]
pub struct ParserArgument<A, B, C, D, E>
where
    A: io::Read,
    B: io::Read,
    C: io::Read,
    D: io::Read,
    E: io::Read,
{
    type_ids: A,
    group_ids: B,
    category_ids: C,
    dogma_attributes: D,
    type_dogma: E,
}

impl<A, B, C, D, E> ParserArgument<A, B, C, D, E>
where
    A: io::Read,
    B: io::Read,
    C: io::Read,
    D: io::Read,
    E: io::Read,
{
    pub fn new(
        type_ids: A,
        group_ids: B,
        category_ids: C,
        dogma_attributes: D,
        type_dogma: E,
    ) -> Self {
        Self {
            type_ids,
            group_ids,
            category_ids,
            dogma_attributes,
            type_dogma,
        }
    }
}

pub type InputSdeData = (
    HashMap<u64, TypeId>,
    HashMap<u64, GroupId>,
    HashMap<u64, CategoryId>,
    HashMap<u64, DogmaAttribute>,
    HashMap<u64, TypeDogma>,
);

pub type OutputSdeData<'a> =
    (HashMap<String, Ship<'a>>, HashMap<String, StaticModule<'a>>);

impl<A, B, C, D, E> Into<InputSdeData> for ParserArgument<A, B, C, D, E>
where
    A: io::Read,
    B: io::Read,
    C: io::Read,
    D: io::Read,
    E: io::Read,
{
    fn into(self) -> InputSdeData {
        (
            serde_yaml::from_reader::<_, HashMap<u64, TypeId>>(self.type_ids)
                .unwrap()
                .into_iter()
                .filter(|(_, x)| x.published)
                .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, GroupId>>(self.group_ids)
                .unwrap()
                .into_iter()
                .filter(|(_, x)| x.published)
                .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, CategoryId>>(
                self.category_ids,
            )
            .unwrap()
            .into_iter()
            .filter(|(_, x)| x.published)
            .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, DogmaAttribute>>(
                self.dogma_attributes,
            )
            .unwrap()
            .into_iter()
            .filter(|(_, x)| x.published)
            .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, TypeDogma>>(
                self.type_dogma,
            )
            .unwrap(),
        )
    }
}

pub fn parse<'a, T: Into<InputSdeData>>(
    input: T,
) -> Result<OutputSdeData<'a>, Box<dyn std::error::Error>> {
    let (type_ids, group_ids, category_ids, dogma_attributes, type_dogmas) =
        input.into();
    let (ships, rest_data): (
        Vec<(
            TypeId,
            &GroupId,
            &CategoryId,
            HashMap<u64, (f64, &DogmaAttribute)>,
        )>,
        Vec<(
            TypeId,
            &GroupId,
            &CategoryId,
            HashMap<u64, (f64, &DogmaAttribute)>,
        )>,
    ) = type_ids
        .into_iter()
        .map(|(i, x)| {
            let group = group_ids.get(&x.group_id)?;
            let category = category_ids.get(&group.category_id)?;
            let type_dogma = type_dogmas.get(&i)?;
            let dogma_attributes = type_dogma
                .dogma_attributes
                .iter()
                .map(|x| {
                    (
                        x.attribute_id,
                        x.value,
                        dogma_attributes.get(&x.attribute_id),
                    )
                })
                .filter(|(_, _, x)| x.is_some())
                .map(|(a, b, c)| (a, (b, c.unwrap())))
                .collect::<HashMap<u64, (f64, &DogmaAttribute)>>();

            Some((x, group, category, dogma_attributes))
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .partition(|(_, _, c, _)| {
            c.name.en.as_ref() == Some(&"Ship".to_string())
        });
    let (modules, _rest_data): (
        Vec<(
            TypeId,
            &GroupId,
            &CategoryId,
            HashMap<u64, (f64, &DogmaAttribute)>,
        )>,
        Vec<(
            TypeId,
            &GroupId,
            &CategoryId,
            HashMap<u64, (f64, &DogmaAttribute)>,
        )>,
    ) = rest_data.into_iter().partition(|(_, _, c, _)| {
        c.name.en.as_ref() == Some(&"Module".to_string())
    });
    let module_map = modules
        .into_iter()
        .map(|(t, g, c, v)| {
            let pg = v
                .get(&549)
                .map(|(x, _)| ModificationType::Additive(*x))
                .unwrap_or(
                    v.get(&145)
                        .map(|(x, _)| ModificationType::Multiplicative(*x))
                        .unwrap_or(
                            v.get(&30)
                                .map(|(x, _)| ModificationType::FittingCost(*x))
                                .unwrap_or(ModificationType::default()),
                        ),
                );
            let cpu = v
                .get(&202)
                .map(|(x, _)| ModificationType::Multiplicative(*x))
                .unwrap_or(
                    v.get(&50)
                        .map(|(x, _)| ModificationType::FittingCost(*x))
                        .unwrap_or(ModificationType::default()),
                );
            let calibration = v
                .get(&1153)
                .map(|(x, _)| ModificationType::FittingCost(*x as u16))
                .unwrap_or(ModificationType::default());
            let cargo = v
                .get(&614)
                .map(|(x, _)| ModificationType::Additive(*x as f32))
                .unwrap_or(
                    v.get(&149)
                        .map(|(x, _)| {
                            ModificationType::Multiplicative(*x as f32)
                        })
                        .unwrap_or(ModificationType::default()),
                );
            let fitting =
                FittingModifications::new(cpu, pg, calibration, cargo);

            let capacitor_amount = v
                .get(&67)
                .map(|(x, _)| ModificationType::Additive(*x))
                .unwrap_or(
                    v.get(&147)
                        .map(|(x, _)| ModificationType::Multiplicative(*x))
                        .unwrap_or(ModificationType::default()),
                );
            let capacitor_recharge_time = v
                .get(&144)
                .map(|(x, _)| ModificationType::Multiplicative(*x))
                .unwrap_or(ModificationType::default());
            //let neut_resistance = v.get(&2267);
            //TODO: neut
            let neut_resistance = ModificationType::<f64>::default();

            let capacitor = CapacitorModifications::new(
                capacitor_amount,
                capacitor_recharge_time,
                neut_resistance,
            );
            let (
                (
                    hull_exp_resists_passive,
                    armor_exp_resists_passive,
                    shield_exp_resists_passive,
                ),
                (
                    hull_exp_resists_active,
                    armor_exp_resists_active,
                    shield_exp_resists_active,
                ),
                active1,
            ) = extract_resistance::extract_resistance(
                &v, g, 975, 268, 272, 985,
            );

            let (
                (
                    hull_em_resists_passive,
                    armor_em_resists_passive,
                    shield_em_resists_passive,
                ),
                (
                    hull_em_resists_active,
                    armor_em_resists_active,
                    shield_em_resists_active,
                ),
                active2,
            ) = extract_resistance::extract_resistance(
                &v, g, 974, 267, 271, 984,
            );

            let (
                (
                    hull_kinetic_resists_passive,
                    armor_kinetic_resists_passive,
                    shield_kinetic_resists_passive,
                ),
                (
                    hull_kinetic_resists_active,
                    armor_kinetic_resists_active,
                    shield_kinetic_resists_active,
                ),
                active3,
            ) = extract_resistance::extract_resistance(
                &v, g, 976, 269, 273, 986,
            );

            let (
                (
                    hull_thermal_resists_passive,
                    armor_thermal_resists_passive,
                    shield_thermal_resists_passive,
                ),
                (
                    hull_thermal_resists_active,
                    armor_thermal_resists_active,
                    shield_thermal_resists_active,
                ),
                active4,
            ) = extract_resistance::extract_resistance(
                &v, g, 977, 270, 274, 987,
            );

            let shield_hp = v
                .get(&337)
                .map(|(x, _)| ModificationType::Multiplicative(*x))
                .unwrap_or(
                    v.get(&72)
                        .map(|(x, _)| ModificationType::Additive(*x))
                        .unwrap_or(ModificationType::default()),
                );
            let (shield_hp_active, shield_hp_passive) = if v.get(&73).is_some()
            {
                (shield_hp, ModificationType::default())
            } else {
                (ModificationType::default(), shield_hp)
            };

            let armor_hp = v
                .get(&335)
                .map(|(x, _)| ModificationType::Multiplicative(*x))
                .unwrap_or(
                    v.get(&1159)
                        .map(|(x, _)| ModificationType::Additive(*x))
                        .unwrap_or(ModificationType::default()),
                );
            let (armor_hp_active, armor_hp_passive) = if v.get(&73).is_some() {
                (armor_hp, ModificationType::default())
            } else {
                (ModificationType::default(), armor_hp)
            };

            let hull_hp = v
                .get(&327)
                .map(|(x, _)| ModificationType::Multiplicative(*x))
                .unwrap_or(ModificationType::default());
            let (hull_hp_active, hull_hp_passive) = if v.get(&73).is_some() {
                (hull_hp, ModificationType::default())
            } else {
                (ModificationType::default(), hull_hp)
            };

            let active_defense = DefenseModifications::new(
                hull_hp_active,
                hull_em_resists_active,
                hull_thermal_resists_active,
                hull_kinetic_resists_active,
                hull_exp_resists_active,
                armor_hp_active,
                armor_em_resists_active,
                armor_thermal_resists_active,
                armor_kinetic_resists_active,
                armor_exp_resists_active,
                shield_hp_active,
                shield_em_resists_active,
                shield_thermal_resists_active,
                shield_kinetic_resists_active,
                shield_exp_resists_active,
                Default::default(),
            );

            let passive_defense = DefenseModifications::new(
                hull_hp_passive,
                hull_em_resists_passive,
                hull_thermal_resists_passive,
                hull_kinetic_resists_passive,
                hull_exp_resists_passive,
                armor_hp_passive,
                armor_em_resists_passive,
                armor_thermal_resists_passive,
                armor_kinetic_resists_passive,
                armor_exp_resists_passive,
                shield_hp_passive,
                shield_em_resists_passive,
                shield_thermal_resists_passive,
                shield_kinetic_resists_passive,
                shield_exp_resists_passive,
                Default::default(),
            );

            StaticModule::new(
                t.name.en.unwrap(),
                fitting,
                capacitor,
                passive_defense,
                active_defense,
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                ModuleSlot::High,
                Default::default(),
                active1 || active2 || active3 || active4,
            )
        })
        .map(|x| (x.name.to_string(), x))
        .collect();
    let ship_map = ships
        .into_iter()
        .map(|(t, g, c, v)| {
            let ship_type = ship_type_by_id(t.group_id);
            (t, g, c, v, ship_type)
        })
        .filter(|(_, _, _, _, ship_type)| ship_type.is_some())
        .map(|(t, g, c, v, ship_type)| (t, g, c, v, ship_type.unwrap()))
        .map(|(t, _g, _c, v, ship_type)| {
            let (low_slots, _) = v.get(&12).unwrap();
            let (med_slots, _) = v.get(&13).unwrap();
            let (high_slots, _) = v.get(&14).unwrap();
            let (shield_hp, _) = v.get(&263).unwrap();
            let (armor_hp, _) = v.get(&265).unwrap();
            let (hull_hp, _) = v.get(&9).unwrap();
            let mass = &t.mass.unwrap();
            let (pg, _) = v.get(&11).unwrap();
            let (cpu, _) = v.get(&48).unwrap();
            let (turret_hard_points, _) = v.get(&101).unwrap();
            let (launcher_hard_points, _) = v.get(&102).unwrap();
            let (rig_slots, _) = v.get(&1154).unwrap();
            let rig_size = match v.get(&1547).unwrap().0 as u8 {
                1 => RigSize::Small,
                2 => RigSize::Medium,
                3 => RigSize::Large,
                4 => RigSize::Capital,
                _ => panic!("couldnt determine rig size"),
            };

            let (sensor_strength_type0, _) = v.get(&208).unwrap();
            let (sensor_strength_type1, _) = v.get(&209).unwrap();
            let (sensor_strength_type2, _) = v.get(&210).unwrap();
            let (sensor_strength_type3, _) = v.get(&211).unwrap();
            let (sensor_strength_type, sensor_strength) =
                if *sensor_strength_type0 as usize != 0 {
                    (SensorStrengthType::Ladar, sensor_strength_type0)
                } else if *sensor_strength_type1 as usize != 0 {
                    (SensorStrengthType::Radar, sensor_strength_type1)
                } else if *sensor_strength_type2 as usize != 0 {
                    (SensorStrengthType::Magnetometric, sensor_strength_type2)
                } else if *sensor_strength_type3 as usize != 0 {
                    (SensorStrengthType::Gravimetric, sensor_strength_type3)
                } else {
                    panic!("couldnt determine sensor strength type")
                };

            let (calibration, _) = v.get(&1132).unwrap();
            let cargo = t.capacity.unwrap();
            let (hull_em_resists, _) = v.get(&113).unwrap();
            let (hull_therm_resists, _) = v.get(&110).unwrap();
            let (hull_kin_resists, _) = v.get(&109).unwrap();
            let (hull_exp_resists, _) = v.get(&111).unwrap();
            let (armor_em_resists, _) = v.get(&267).unwrap();
            let (armor_therm_resists, _) = v.get(&270).unwrap();
            let (armor_kin_resists, _) = v.get(&269).unwrap();
            let (armor_exp_resists, _) = v.get(&268).unwrap();
            let (shield_em_resists, _) = v.get(&271).unwrap();
            let (shield_therm_resists, _) = v.get(&274).unwrap();
            let (shield_kin_resists, _) = v.get(&273).unwrap();
            let (shield_exp_resists, _) = v.get(&272).unwrap();
            let (sig_radius, _) = v.get(&552).unwrap();

            let (velocity, _) = v.get(&37).unwrap();
            let (agility, _) = v.get(&70).unwrap();
            let (warp_speed, _) = v.get(&600).unwrap();

            let (targeting_range, _) = v.get(&76).unwrap();
            let (scan_res, _) = v.get(&564).unwrap();
            let (max_locked_targets, _) = v.get(&192).unwrap();

            let control_range = 20u32;
            let (capacity, _) = v.get(&283).unwrap();
            let (bandwidth, _) = v.get(&1271).unwrap();
            let max_drones = 0u8;

            let (capacitor_amount, _) = v.get(&482).unwrap();
            let (capacitor_recharge_time, _) = v.get(&55).unwrap();
            let neut_resistance = 0f32;

            let name = t.name.en.unwrap();
            let faction = match t.faction_id {
                //some entries randomly dont have factions attached, so we harded them :shrug:
                None => match name.as_str() {
                    "Whiptail" | "Chameleon" => Faction::Guristas,
                    "Pacifier" | "Marshal" | "Enforcer" => Faction::Concord,
                    "Stratios Emergency Responder" => Faction::SistersOfEve,
                    _ => panic!("couldnt determine faction for ship {} ", name),
                },
                Some(x) => faction_by_id(x),
            };
            Ship::new(
                name,
                ship_type,
                faction,
                *high_slots as u8,
                *med_slots as u8,
                *low_slots as u8,
                *turret_hard_points as u8,
                *launcher_hard_points as u8,
                *rig_slots as u8,
                rig_size,
                sensor_strength_type,
                Fitting::new(*cpu, *pg, *calibration as u16, cargo as f32),
                Defense::new(
                    *hull_hp,
                    *hull_em_resists,
                    *hull_therm_resists,
                    *hull_kin_resists,
                    *hull_exp_resists,
                    *armor_hp,
                    *armor_em_resists,
                    *armor_therm_resists,
                    *armor_kin_resists,
                    *armor_exp_resists,
                    *shield_hp,
                    *shield_em_resists,
                    *shield_therm_resists,
                    *shield_kin_resists,
                    *shield_exp_resists,
                    *sig_radius,
                ),
                Movement::new(*velocity as f64, *agility, *mass, *warp_speed),
                Sensor::new(
                    *targeting_range,
                    *scan_res,
                    *sensor_strength,
                    *max_locked_targets as u8,
                ),
                Drone::new(
                    control_range,
                    *capacity as u16,
                    *bandwidth as u16,
                    max_drones,
                ),
                Capacitor::new(
                    *capacitor_amount,
                    *capacitor_recharge_time,
                    neut_resistance as f64,
                ),
            )
        })
        .map(|x| (x.name.to_string(), x))
        .collect();
    Ok((ship_map, module_map))
}
