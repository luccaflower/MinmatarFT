use fitting_engine::ship_type::battlecruiser::BattlecruiserType;
use fitting_engine::ship_type::battleship::BattleshipType;
use fitting_engine::ship_type::cruiser::CruiserType;
use fitting_engine::ship_type::destroyer::DestroyerType;
use fitting_engine::ship_type::frigate::FrigateType;
use fitting_engine::ship_type::ShipType;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct ShipTypeWrapper(ShipType);

impl ShipTypeWrapper {
    pub fn new(ship_type: ShipType) -> Self {
        Self(ship_type)
    }
}

impl ToTokens for ShipTypeWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lok = match &self.0 {
            ShipType::Frigate(x) => match x {
                FrigateType::T1 => quote::quote! {
                    fitting_engine::ship_type::ShipType::Frigate(fitting_engine::ship_type::frigate::FrigateType::T1)
                },
                FrigateType::Interceptor => quote::quote! {
                    fitting_engine::ship_type::ShipType::Frigate(fitting_engine::ship_type::frigate::FrigateType::Interceptor)
                },
                FrigateType::Assault => quote::quote! {
                    fitting_engine::ship_type::ShipType::Frigate(fitting_engine::ship_type::frigate::FrigateType::Assault)
                },
                FrigateType::ElectronicAttack => quote::quote! {
                    fitting_engine::ship_type::ShipType::Frigate(fitting_engine::ship_type::frigate::FrigateType::ElectronicAttack)
                },
                FrigateType::Logistics => quote::quote! {
                    fitting_engine::ship_type::ShipType::Frigate(fitting_engine::ship_type::frigate::FrigateType::Logistics)
                },
            },
            ShipType::Destroyer(x) => match x {
                DestroyerType::T1 => quote::quote! {
                    fitting_engine::ship_type::ShipType::Destroyer(fitting_engine::ship_type::destroyer::DestroyerType::T1)
                },
                DestroyerType::Interdictor => quote::quote! {
                    fitting_engine::ship_type::ShipType::Destroyer(fitting_engine::ship_type::destroyer::DestroyerType::Interdictor)
                },
                DestroyerType::Command => quote::quote! {
                    fitting_engine::ship_type::ShipType::Destroyer(fitting_engine::ship_type::destroyer::DestroyerType::Command)
                },
                DestroyerType::Tactical => quote::quote! {
                    fitting_engine::ship_type::ShipType::Destroyer(fitting_engine::ship_type::destroyer::DestroyerType::Tactical)
                },
            },
            ShipType::Cruiser(x) => match x {
                CruiserType::T1 => quote::quote! {
                    fitting_engine::ship_type::ShipType::Cruiser(fitting_engine::ship_type::cruiser::CruiserType::T1)
                },
                CruiserType::HeavyAssault => quote::quote! {
                    fitting_engine::ship_type::ShipType::Cruiser(fitting_engine::ship_type::cruiser::CruiserType::HeavyAssault)
                },
                CruiserType::HeavyInterdictor => quote::quote! {
                    fitting_engine::ship_type::ShipType::Cruiser(fitting_engine::ship_type::cruiser::CruiserType::HeavyInterdictor)
                },
                CruiserType::Logistics => quote::quote! {
                    fitting_engine::ship_type::ShipType::Cruiser(fitting_engine::ship_type::cruiser::CruiserType::Logistics)
                },
                CruiserType::Strategic => quote::quote! {
                    fitting_engine::ship_type::ShipType::Cruiser(fitting_engine::ship_type::cruiser::CruiserType::Strategic)
                },
                CruiserType::ForceRecon => quote::quote! {
                    fitting_engine::ship_type::ShipType::Cruiser(fitting_engine::ship_type::cruiser::CruiserType::ForceRecon)
                },
                CruiserType::CombatRecon => quote::quote! {
                    fitting_engine::ship_type::ShipType::Cruiser(fitting_engine::ship_type::cruiser::CruiserType::CombatRecon)
                },
            },
            ShipType::Battlecruiser(x) => match x {
                BattlecruiserType::T1 => quote::quote! {
                    fitting_engine::ship_type::ShipType::Battlecruiser(fitting_engine::ship_type::battlecruiser::BattlecruiserType::T1)
                },
                BattlecruiserType::Command => quote::quote! {
                    fitting_engine::ship_type::ShipType::Battlecruiser(fitting_engine::ship_type::battlecruiser::BattlecruiserType::Command)
                },
            },
            ShipType::Battleship(x) => match x {
                BattleshipType::T1 => quote::quote! {
                    fitting_engine::ship_type::ShipType::Battleship(fitting_engine::ship_type::battleship::BattleshipType::T1)
                },
                BattleshipType::Marauder => quote::quote! {
                    fitting_engine::ship_type::ShipType::Battleship(fitting_engine::ship_type::battleship::BattleshipType::Marauder)
                },
            },
        };
        lok.to_tokens(tokens);
    }
}
