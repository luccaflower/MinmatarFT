use crate::faction_ids::faction_by_id;
use crate::model::category_id::CategoryId;
use crate::model::dogma_attribute::DogmaAttribute;
use crate::model::group_id::GroupId;
use crate::model::type_dogma::TypeDogma;
use crate::model::type_id::TypeId;
use crate::ship_type_ids::ship_type_by_id;
use fitting_engine::faction::Faction;
use fitting_engine::ship::{RigSize, SensorStrengthType, Ship};
use fitting_engine::stats::capacitor::Capacitor;
use fitting_engine::stats::defense::Defense;
use fitting_engine::stats::drone::Drone;
use fitting_engine::stats::fitting::Fitting;
use fitting_engine::stats::movement::Movement;
use fitting_engine::stats::sensor::Sensor;
use std::collections::HashMap;
use std::io;

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

pub type OutputSdeData<'a> = (HashMap<String, Ship<'a>>);

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
            serde_yaml::from_reader::<_, HashMap<u64, CategoryId>>(self.category_ids)
                .unwrap()
                .into_iter()
                .filter(|(_, x)| x.published)
                .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, DogmaAttribute>>(self.dogma_attributes)
                .unwrap()
                .into_iter()
                .filter(|(_, x)| x.published)
                .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, TypeDogma>>(self.type_dogma).unwrap(),
        )
    }
}

pub fn parse<'a, T: Into<InputSdeData>>(
    input: T,
) -> Result<OutputSdeData<'a>, Box<dyn std::error::Error>> {
    let (type_ids, group_ids, category_ids, dogma_attributes, type_dogmas) = input.into();
    let (ships, _rest_data): (
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
        .partition(|(_, _, c, _)| c.name.en.as_ref() == Some(&"Ship".to_string()));
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
            let (sensor_strength_type, sensor_strength) = if *sensor_strength_type0 as usize != 0 {
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

            let control_range = 20f32;
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
                Fitting::new(*cpu, *pg, *calibration as u8, cargo as f32),
                Defense::new(
                    *hull_hp as u32,
                    *hull_em_resists as f32,
                    *hull_therm_resists as f32,
                    *hull_kin_resists as f32,
                    *hull_exp_resists as f32,
                    *armor_hp as u32,
                    *armor_em_resists as f32,
                    *armor_therm_resists as f32,
                    *armor_kin_resists as f32,
                    *armor_exp_resists as f32,
                    *shield_hp as u32,
                    *shield_em_resists as f32,
                    *shield_therm_resists as f32,
                    *shield_kin_resists as f32,
                    *shield_exp_resists as f32,
                    *sig_radius as u16,
                ),
                Movement::new(
                    *velocity as u32,
                    *agility as f32,
                    *mass as u64,
                    *warp_speed as f32,
                ),
                Sensor::new(
                    *targeting_range as f32,
                    *scan_res as u16,
                    *sensor_strength as f32,
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
                    *capacitor_recharge_time as u16,
                    neut_resistance,
                ),
            )
        })
        .map(|x| (x.name.to_string(), x))
        .collect();
    Ok(ship_map)
}
