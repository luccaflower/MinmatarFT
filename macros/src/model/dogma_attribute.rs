use crate::model::Localization;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DogmaAttribute {
    #[serde(alias = "categoryID")]
    pub category_id: Option<u64>,
    #[serde(alias = "dataType")]
    pub data_type: u64,
    #[serde(alias = "defaultValue")]
    pub default_value: f64,
    pub description: Option<String>,
    #[serde(alias = "displayNameID")]
    pub display_name_id: Option<Localization>,
    #[serde(alias = "highIsGood")]
    pub high_is_good: bool,
    #[serde(alias = "iconID")]
    pub icon_id: Option<u64>,
    pub name: String,
    pub published: bool,
    pub stackable: bool,
    #[serde(alias = "unitID")]
    pub unit_id: Option<u64>,
}
