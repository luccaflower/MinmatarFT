use crate::domain_impl::faction::FactionWrapper;
use crate::domain_impl::ship_stats::ShipStatsWrapper;
use crate::domain_impl::ship_type::ShipTypeWrapper;
use fitting_engine::ship::Ship;
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
        let ship_stats = ShipStatsWrapper::new(self.0.ship_stats.clone());
        let lok = quote::quote! {
            fitting_engine::ship::Ship {
                name: Cow::Borrowed(#name),
                ship_type: #ship_type,
                faction: #faction,
                high_slots: #high_slots,
                med_slots: #med_slots,
                low_slots: #low_slots,
                ship_stats: #ship_stats,
            }
        };
        lok.to_tokens(tokens);
    }
}
