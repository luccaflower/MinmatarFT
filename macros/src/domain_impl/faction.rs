use quote::ToTokens;
use proc_macro2::TokenStream;
use domain::faction::Faction;

#[derive(Debug, Clone)]
pub struct FactionWrapper(Faction);

impl FactionWrapper {
    pub fn new(faction: Faction) -> Self{
        Self(faction)
    }
}

impl ToTokens for FactionWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tok = match &self.0 {
            Faction::Amarr => quote::quote! {
                domain::faction::Faction::Amarr
            },
            Faction::Minmatar => quote::quote! {
                domain::faction::Faction::Minmatar
            },
            Faction::Caldari => quote::quote! {
                domain::faction::Faction::Caldari
            },
            Faction::Gallente => quote::quote! {
                domain::faction::Faction::Gallente
            },
            Faction::AngelCartel => quote::quote! {
                domain::faction::Faction::AngelCartel
            },
            Faction::MordusLegion => quote::quote! {
                domain::faction::Faction::MordusLegion
            },
            Faction::Guristas => quote::quote! {
                domain::faction::Faction::Guristas
            },
            Faction::BloodRaiders => quote::quote! {
                domain::faction::Faction::BloodRaiders
            },
            Faction::SanshasNation => quote::quote! {
                domain::faction::Faction::SanshasNation
            },
            Faction::Serpentis => quote::quote! {
                domain::faction::Faction::Serpentis
            },
            Faction::Triglavians => quote::quote! {
                domain::faction::Faction::Triglavians
            },
            Faction::SistersOfEve => quote::quote! {
                domain::faction::Faction::SistersOfEve
            },
        };
        tok.to_tokens(tokens);
    }
}
