use crate::model::Localization;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryId {
    pub name: Localization,
    #[serde(alias = "iconID")]
    pub icon_id: Option<u64>,
    pub published: bool,
}
