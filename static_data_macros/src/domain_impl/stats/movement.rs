use fitting_engine::stats::movement::Movement;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct MovementWrapper(Movement);

impl MovementWrapper {
    pub fn new(c: Movement) -> Self {
        Self(c)
    }
}

impl ToTokens for MovementWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Movement {
            max_velocity,
            agility,
            mass,
            warp_speed,
        } = self.0;
        let tok = quote::quote! {
            fitting_engine::stats::movement::Movement {
                max_velocity: #max_velocity,
                agility: #agility,
                mass: #mass,
                warp_speed: #warp_speed,
            }
        };
        tok.to_tokens(tokens)
    }
}
