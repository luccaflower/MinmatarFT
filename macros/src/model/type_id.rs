use crate::model::Localization;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TypeId {
    pub capacity: Option<f64>,
    pub description: Option<Localization>,
    #[serde(alias = "factionID")]
    pub faction_id: Option<u64>,
    #[serde(alias = "groupID")]
    pub group_id: u64,
    #[serde(alias = "graphicID")]
    pub graphic_id: Option<u64>,
    #[serde(alias = "market_group_id")]
    pub market_group_id: Option<u64>,
    pub mass: Option<f64>,
    pub masteries: Option<HashMap<u64, Vec<u64>>>,
    #[serde(alias = "metaGroupID")]
    pub meta_group_id: Option<u64>,
    pub name: Localization,
    #[serde(alias = "portionSize")]
    pub portion_size: u64,
    pub published: bool,
    #[serde(alias = "raceID")]
    pub race_id: Option<u64>,
    pub radius: Option<f64>,
    #[serde(alias = "sofFactionName")]
    pub sof_faction_name: Option<String>,
    #[serde(alias = "soundID")]
    pub sound_id: Option<String>,
    pub traits: Option<Trait>,
    pub volume: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Trait {
    pub types: Option<HashMap<u64, Vec<Type>>>,
    #[serde(alias = "roleBonuses")]
    pub role_bonuses: Option<Vec<Type>>,
    #[serde(alias = "miscBonuses")]
    pub misc_bonuses: Option<Vec<Type>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Type {
    pub bonus: Option<f64>,
    #[serde(alias = "bonusText")]
    pub bonus_text: Localization,
    pub importance: u64,
    #[serde(alias = "unitID")]
    pub unit_id: Option<u64>,
    #[serde(alias = "isPositive")]
    pub is_positive: Option<bool>,
}
