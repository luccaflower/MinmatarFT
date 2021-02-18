use crate::ship_type::stat_modification::StatModification;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module<'a> {
    pub name: Cow<'a, str>,
    pub active_effect: Option<StatModification>,
    pub passive_effect: StatModification,
    //TODO: module_stats: ModuleStats
}

impl Module<'_> {
    pub fn active(&self) -> bool {
        self.active_effect.is_some()
    }
}
