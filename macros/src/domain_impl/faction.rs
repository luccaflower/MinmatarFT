use fitting_engine::faction::Faction;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct FactionWrapper(Faction);

impl FactionWrapper {
    pub fn new(faction: Faction) -> Self {
        Self(faction)
    }
}

impl ToTokens for FactionWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tok = match &self.0 {
            Faction::Amarr => quote::quote! {
                fitting_engine::faction::Faction::Amarr
            },
            Faction::Minmatar => quote::quote! {
                fitting_engine::faction::Faction::Minmatar
            },
            Faction::Caldari => quote::quote! {
                fitting_engine::faction::Faction::Caldari
            },
            Faction::Gallente => quote::quote! {
                fitting_engine::faction::Faction::Gallente
            },
            Faction::AngelCartel => quote::quote! {
                fitting_engine::faction::Faction::AngelCartel
            },
            Faction::MordusLegion => quote::quote! {
                fitting_engine::faction::Faction::MordusLegion
            },
            Faction::Guristas => quote::quote! {
                fitting_engine::faction::Faction::Guristas
            },
            Faction::BloodRaiders => quote::quote! {
                fitting_engine::faction::Faction::BloodRaiders
            },
            Faction::SanshasNation => quote::quote! {
                fitting_engine::faction::Faction::SanshasNation
            },
            Faction::Serpentis => quote::quote! {
                fitting_engine::faction::Faction::Serpentis
            },
            Faction::Triglavians => quote::quote! {
                fitting_engine::faction::Faction::Triglavians
            },
            Faction::SistersOfEve => quote::quote! {
                fitting_engine::faction::Faction::SistersOfEve
            },
            Faction::SocietyofConsciousThought => quote::quote! {
                fitting_engine::faction::Faction::SocietyofConsciousThought
            },
            Faction::EdenCom => quote::quote! {
                fitting_engine::faction::Faction::EdenCom
            },
        };
        tok.to_tokens(tokens);
    }
}
