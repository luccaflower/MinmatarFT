use crate::faction_ids::faction_by_id;
use crate::model::category_id::CategoryId;
use crate::model::dogma_attribute::DogmaAttribute;
use crate::model::group_id::GroupId;
use crate::model::type_dogma::TypeDogma;
use crate::model::type_id::TypeId;
use crate::ship_type_ids::ship_type_by_id;
use fitting_engine::faction::Faction;
use fitting_engine::ship::Ship;
use fitting_engine::ship_stats::ShipStats;
use std::collections::{HashMap, HashSet};
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
                .filter(|(_, x)| x.published.clone())
                .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, GroupId>>(self.group_ids)
                .unwrap()
                .into_iter()
                .filter(|(_, x)| x.published.clone())
                .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, CategoryId>>(self.category_ids)
                .unwrap()
                .into_iter()
                .filter(|(_, x)| x.published.clone())
                .collect(),
            serde_yaml::from_reader::<_, HashMap<u64, DogmaAttribute>>(self.dogma_attributes)
                .unwrap()
                .into_iter()
                .filter(|(_, x)| x.published.clone())
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
                        x.value.clone(),
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
    let mut attributes = HashMap::new();
    let ship_map = ships
        .into_iter()
        .map(|(t, g, c, v)| {
            let ship_type = ship_type_by_id(t.group_id.clone());
            (t, g, c, v, ship_type)
        })
        .filter(|(_, _, _, _, ship_type)| ship_type.is_some())
        .map(|(t, g, c, v, ship_type)| (t, g, c, v, ship_type.unwrap()))
        .map(|(t, g, c, v, ship_type)| {
            for a in &v {
                attributes.insert(a.0.clone(), a.1 .1.clone());
            }
            let (low_slots, _) = v.get(&12).unwrap();
            let (med_slots, _) = v.get(&13).unwrap();
            let (high_slots, _) = v.get(&14).unwrap();
            let (shield_hp, _) = v.get(&263).unwrap();
            let (armor_hp, _) = v.get(&265).unwrap();
            let (hull_hp, _) = v.get(&9).unwrap();
            let (velocity, _) = v.get(&37).unwrap();
            let (agility, _) = v.get(&70).unwrap();
            let mass = &t.mass.unwrap();
            let (power_grid, _) = v.get(&11).unwrap();
            let (cpu, _) = v.get(&48).unwrap();

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
                high_slots.clone() as u8,
                med_slots.clone() as u8,
                low_slots.clone() as u8,
                ShipStats::new(
                    shield_hp.clone() as usize,
                    armor_hp.clone() as usize,
                    hull_hp.clone() as usize,
                    velocity.clone() as usize,
                    agility.clone() as usize,
                    mass.clone() as usize,
                    power_grid.clone() as usize,
                    cpu.clone() as usize,
                ),
            )
        })
        .map(|x| (x.name.to_string(), x))
        .collect();
    println!("{:?}", attributes);
    Ok(ship_map)
}
