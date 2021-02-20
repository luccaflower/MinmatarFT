use fitting_engine::stats::fitting::Fitting;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct FittingWrapper(Fitting);

impl FittingWrapper {
    pub fn new(c: Fitting) -> Self {
        Self(c)
    }
}

impl ToTokens for FittingWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Fitting {
            cpu,
            pg,
            calibration,
            cargo,
        } = self.0;
        let tok = quote::quote! {
            fitting_engine::stats::fitting::Fitting {
                cpu: #cpu,
                pg: #pg,
                calibration: #calibration,
                cargo: #cargo,
            }
        };
        tok.to_tokens(tokens)
    }
}
