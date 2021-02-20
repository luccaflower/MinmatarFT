use fitting_engine::stats::capacitor::Capacitor;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct CapacitorWrapper(Capacitor);

impl CapacitorWrapper {
    pub fn new(c: Capacitor) -> Self {
        Self(c)
    }
}

impl ToTokens for CapacitorWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Capacitor {
            capacitor_amount,
            capacitor_recharge_time,
            neut_resistance,
        } = self.0;
        let tok = quote::quote! {
            fitting_engine::stats::capacitor::Capacitor {
                capacitor_amount: #capacitor_amount,
                capacitor_recharge_time: #capacitor_recharge_time,
                neut_resistance: #neut_resistance,
            }
        };
        tok.to_tokens(tokens)
    }
}
