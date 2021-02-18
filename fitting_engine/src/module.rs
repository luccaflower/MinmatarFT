use crate::ship_type::stat_modification::StatModification;

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    active_effect: Option<StatModification>,
    passive_effect: StatModification,
    //TODO: module_stats: ModuleStats
}

impl Module {
    pub fn active(&self) -> bool {
        self.active_effect.is_some()
    }
}
