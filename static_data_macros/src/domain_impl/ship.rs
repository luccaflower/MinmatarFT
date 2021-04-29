use crate::domain_impl::faction::FactionWrapper;
use crate::domain_impl::ship_type::ShipTypeWrapper;
use crate::domain_impl::stats::capacitor::CapacitorWrapper;
use crate::domain_impl::stats::defense::DefenseWrapper;
use crate::domain_impl::stats::drone::DroneWrapper;
use crate::domain_impl::stats::fitting::FittingWrapper;
use crate::domain_impl::stats::movement::MovementWrapper;
use crate::domain_impl::stats::sensor::SensorWrapper;
use fitting_engine::ship::{RigSize, SensorStrengthType, Ship};
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct ShipWrapper<'a>(Ship<'a>);

impl<'a> ShipWrapper<'a> {
    pub fn new(ship: Ship<'a>) -> Self {
        Self(ship)
    }
}

impl ToTokens for ShipWrapper<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.0.name.to_string();
        let ship_type = ShipTypeWrapper::new(self.0.ship_type.clone());
        let faction = FactionWrapper::new(self.0.faction.clone());

        let high_slots = self.0.high_slots.clone();
        let med_slots = self.0.med_slots.clone();
        let low_slots = self.0.low_slots.clone();
        let turret_hard_points = self.0.turret_hard_points.clone();
        let launcher_hard_points = self.0.launcher_hard_points.clone();
        let rig_slots = self.0.rig_slots.clone();
        let rig_size = RigSizeWrapper::new(self.0.rig_size.clone());
        let sensor_strength_type =
            SensorStrengthTypeWrapper::new(self.0.sensor_strength_type.clone());

        let fitting_stats = FittingWrapper::new(self.0.fitting_stats.clone());
        let defensive_stats =
            DefenseWrapper::new(self.0.defensive_stats.clone());
        let movement_stats =
            MovementWrapper::new(self.0.movement_stats.clone());
        let sensor_stats = SensorWrapper::new(self.0.sensor_stats.clone());
        let drone_stats = DroneWrapper::new(self.0.drone_stats.clone());
        let capacitor_stats =
            CapacitorWrapper::new(self.0.capacitor_stats.clone());

        let lok = quote::quote! {
            fitting_engine::ship::Ship {
                name: Cow::Borrowed(#name),
                ship_type: #ship_type,
                faction: #faction,

                high_slots: #high_slots,
                med_slots: #med_slots,
                low_slots: #low_slots,
                turret_hard_points: #turret_hard_points,
                launcher_hard_points: #launcher_hard_points,
                rig_slots: #rig_slots,
                rig_size: #rig_size,
                sensor_strength_type: #sensor_strength_type,

                fitting_stats: #fitting_stats,
                defensive_stats: #defensive_stats,
                movement_stats: #movement_stats,
                sensor_stats: #sensor_stats,
                drone_stats: #drone_stats,
                capacitor_stats: #capacitor_stats,
            }
        };
        lok.to_tokens(tokens);
    }
}

#[derive(Debug, Clone)]
pub struct RigSizeWrapper(RigSize);

impl RigSizeWrapper {
    pub fn new(c: RigSize) -> Self {
        Self(c)
    }
}

impl ToTokens for RigSizeWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tok = match &self.0 {
            RigSize::Small => quote::quote! {
                fitting_engine::ship::RigSize::Small
            },
            RigSize::Medium => quote::quote! {
                fitting_engine::ship::RigSize::Medium
            },
            RigSize::Large => quote::quote! {
                fitting_engine::ship::RigSize::Large
            },
            RigSize::Capital => quote::quote! {
                fitting_engine::ship::RigSize::Capital
            },
        };
        tok.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
pub struct SensorStrengthTypeWrapper(SensorStrengthType);

impl SensorStrengthTypeWrapper {
    pub fn new(c: SensorStrengthType) -> Self {
        Self(c)
    }
}

impl ToTokens for SensorStrengthTypeWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tok = match &self.0 {
            SensorStrengthType::Ladar => quote::quote! {
                fitting_engine::ship::SensorStrengthType::Ladar
            },
            SensorStrengthType::Radar => quote::quote! {
                fitting_engine::ship::SensorStrengthType::Radar
            },
            SensorStrengthType::Magnetometric => quote::quote! {
                fitting_engine::ship::SensorStrengthType::Magnetometric
            },
            SensorStrengthType::Gravimetric => quote::quote! {
                fitting_engine::ship::SensorStrengthType::Gravimetric
            },
        };
        tok.to_tokens(tokens)
    }
}
