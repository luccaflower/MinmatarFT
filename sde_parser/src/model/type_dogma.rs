use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDogma {
    #[serde(alias = "dogmaAttributes")]
    pub dogma_attributes: Vec<DogmaAttributeEntry>,
    #[serde(alias = "dogmaEffects")]
    pub dogma_effects: Vec<DogmaEffectEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DogmaAttributeEntry {
    #[serde(alias = "attributeID")]
    pub attribute_id: u64,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DogmaEffectEntry {
    #[serde(alias = "effectID")]
    pub effect_id: u64,
    pub value: Option<f64>,
    #[serde(alias = "isDefault")]
    pub is_default: Option<bool>,
}
