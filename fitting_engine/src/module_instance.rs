use crate::static_module::{
    CapacitorMod, DefenseMod, DroneMod, FittingMod, MovementMod, SensorMod,
    StaticModule,
};

#[derive(Debug, Clone)]
pub struct ModuleInstance<'a> {
    pub inner_module: &'a StaticModule<'a>,
    pub state: ModuleInstanceState,
}

impl<'a> ModuleInstance<'a> {
    pub fn new(inner_module: &'a StaticModule<'a>) -> Self {
        Self {
            inner_module,
            state: ModuleInstanceState::Passive,
        }
    }

    pub fn passive(&mut self) {
        self.state = ModuleInstanceState::Passive;
    }

    pub fn active(&mut self) {
        self.state = ModuleInstanceState::Active;
    }

    pub fn overheat(&mut self) {
        self.state = ModuleInstanceState::Overheat;
    }

    pub fn modifications(
        &self,
    ) -> (
        &FittingMod,
        &CapacitorMod,
        &DefenseMod,
        &MovementMod,
        &SensorMod,
        &DroneMod,
    ) {
        match self.state {
            ModuleInstanceState::Passive => self.inner_module.passive_mods(),
            ModuleInstanceState::Active => self.inner_module.active_mods(),
            ModuleInstanceState::Overheat => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModuleInstanceState {
    Passive,
    Active,
    Overheat,
}

impl Default for ModuleInstanceState {
    fn default() -> Self {
        Self::Passive
    }
}
