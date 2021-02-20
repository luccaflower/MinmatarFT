use fitting_engine::stats::sensor::Sensor;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct SensorWrapper(Sensor);

impl SensorWrapper {
    pub fn new(c: Sensor) -> Self {
        Self(c)
    }
}

impl ToTokens for SensorWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Sensor {
            targeting_range,
            scan_res,
            sensor_strength,
            max_locked_targets,
        } = self.0;
        let tok = quote::quote! {
            fitting_engine::stats::sensor::Sensor {
                targeting_range: #targeting_range,
                scan_res: #scan_res,
                sensor_strength: #sensor_strength,
                max_locked_targets: #max_locked_targets,
            }
        };
        tok.to_tokens(tokens)
    }
}
