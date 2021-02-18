use quote::ToTokens;
use proc_macro2::TokenStream;
use domain::ship_type::ShipType;
use domain::ship_type::frigate::FrigateType;
use domain::ship_type::destroyer::DestroyerType;
use domain::ship_type::cruiser::CruiserType;
use domain::ship_type::battlecruiser::BattlecruiserType;
use domain::ship_type::battleship::BattleshipType;

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
                    domain::ship_type::ShipType::Frigate(domain::ship_type::frigate::FrigateType::T1)
                },
                FrigateType::Interceptor => quote::quote! {
                    domain::ship_type::ShipType::Frigate(domain::ship_type::frigate::FrigateType::Interceptor)
                },
                FrigateType::Assault => quote::quote! {
                    domain::ship_type::ShipType::Frigate(domain::ship_type::frigate::FrigateType::Assault)
                }
            }
            ShipType::Destroyer(x) => match x {
                DestroyerType::T1 => quote::quote! {
                    domain::ship_type::ShipType::Destroyer(domain::ship_type::destroyer::DestroyerType::T1)
                },
                DestroyerType::Interdictor => quote::quote! {
                    domain::ship_type::ShipType::Destroyer(domain::ship_type::destroyer::DestroyerType::Interdictor)
                },
                DestroyerType::Command => quote::quote! {
                    domain::ship_type::ShipType::Destroyer(domain::ship_type::destroyer::DestroyerType::Command)
                },
                DestroyerType::Tactical => quote::quote! {
                    domain::ship_type::ShipType::Destroyer(domain::ship_type::destroyer::DestroyerType::Tactical)
                }
            }
            ShipType::Cruiser(x) => match x {
                CruiserType::T1 => quote::quote! {
                    domain::ship_type::ShipType::Cruiser(domain::ship_type::cruiser::CruiserType::T1)
                },
                CruiserType::HeavyAssault => quote::quote! {
                    domain::ship_type::ShipType::Cruiser(domain::ship_type::cruiser::CruiserType::HeavyAssault)
                },
                CruiserType::HeavyInterdictor => quote::quote! {
                    domain::ship_type::ShipType::Cruiser(domain::ship_type::cruiser::CruiserType::HeavyInterdictor)
                },
                CruiserType::Logistics => quote::quote! {
                    domain::ship_type::ShipType::Cruiser(domain::ship_type::cruiser::CruiserType::Logistics)
                },
                CruiserType::Strategic => quote::quote! {
                    domain::ship_type::ShipType::Cruiser(domain::ship_type::cruiser::CruiserType::Strategic)
                }
            }
            ShipType::Battlecruiser(x) => match x {
                BattlecruiserType::T1 => quote::quote! {
                    domain::ship_type::ShipType::Battlecruiser(domain::ship_type::battlecruiser::BattlecruiserType::T1)
                },
                BattlecruiserType::Command => quote::quote! {
                    domain::ship_type::ShipType::Battlecruiser(domain::ship_type::battlecruiser::BattlecruiserType::Command)
                }
            }
            ShipType::Battleship(x) => match x {
                BattleshipType::T1 => quote::quote! {
                    domain::ship_type::ShipType::Battleship(domain::ship_type::battleship::BattleshipType::T1)
                },
                BattleshipType::Marauder => quote::quote! {
                    domain::ship_type::ShipType::Battleship(domain::ship_type::battleship::BattleshipType::Marauder)
                }
            }
        };
        lok.to_tokens(tokens);
    }
}
