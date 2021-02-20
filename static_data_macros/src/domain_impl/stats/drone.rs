use fitting_engine::stats::drone::Drone;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct DroneWrapper(Drone);

impl DroneWrapper {
    pub fn new(c: Drone) -> Self {
        Self(c)
    }
}

impl ToTokens for DroneWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Drone {
            control_range,
            capacity,
            bandwidth,
            max_drones,
        } = self.0;
        let tok = quote::quote! {
            fitting_engine::stats::drone::Drone {
                control_range: #control_range,
                capacity: #capacity,
                bandwidth: #bandwidth,
                max_drones: #max_drones,
            }
        };
        tok.to_tokens(tokens)
    }
}
