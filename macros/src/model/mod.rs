pub mod category_id;
pub mod dogma_attribute;
pub mod group_id;
pub mod type_dogma;
pub mod type_id;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Localization {
    pub de: Option<String>,
    pub en: Option<String>,
    pub fr: Option<String>,
    pub ja: Option<String>,
    pub ru: Option<String>,
    pub zh: Option<String>,
}
