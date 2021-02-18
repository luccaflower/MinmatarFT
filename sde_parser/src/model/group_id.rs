use crate::model::Localization;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GroupId {
    pub anchorable: bool,
    pub anchored: bool,
    #[serde(alias = "categoryID")]
    pub category_id: u64,
    #[serde(alias = "fittableNonSingleton")]
    pub fittable_non_singleton: bool,
    #[serde(alias = "iconID")]
    pub icon_id: Option<u64>,
    pub name: Localization,
    pub published: bool,
    #[serde(alias = "useBasePrice")]
    pub use_base_price: bool,
}
