use domain::ship_stats::ShipStats;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct ShipStatsWrapper(ShipStats);

impl ShipStatsWrapper {
    pub fn new(ship_stats: ShipStats) -> Self {
        Self(ship_stats)
    }
}

impl ToTokens for ShipStatsWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ShipStats {
            shield_hp,
            armor_hp,
            hull_hp,
            velocity,
            agility,
            mass,
            power_grid,
            cpu,
        } = self.0.clone();
        let lok = quote::quote! {
            domain::ship_stats::ShipStats {
                shield_hp: #shield_hp,
                armor_hp: #armor_hp,
                hull_hp: #hull_hp,
                velocity: #velocity,
                agility: #agility,
                mass: #mass,
                power_grid: #power_grid,
                cpu: #cpu,
            }
        };
        lok.to_tokens(tokens);
    }
}
