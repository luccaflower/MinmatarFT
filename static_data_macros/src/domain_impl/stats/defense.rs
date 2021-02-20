use fitting_engine::stats::defense::Defense;
use proc_macro2::TokenStream;
use quote::ToTokens;

#[derive(Debug, Clone)]
pub struct DefenseWrapper(Defense);

impl DefenseWrapper {
    pub fn new(c: Defense) -> Self {
        Self(c)
    }
}

impl ToTokens for DefenseWrapper {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Defense {
            hull_hp,
            hull_em_resists,
            hull_therm_resists,
            hull_kin_resists,
            hull_exp_resists,
            armor_hp,
            armor_em_resists,
            armor_therm_resists,
            armor_kin_resists,
            armor_exp_resists,
            shield_hp,
            shield_em_resists,
            shield_therm_resists,
            shield_kin_resists,
            shield_exp_resists,
            sig_radius,
        } = self.0;
        let tok = quote::quote! {
            fitting_engine::stats::defense::Defense {
                hull_hp: #hull_hp,
                hull_em_resists: #hull_em_resists,
                hull_therm_resists: #hull_therm_resists,
                hull_kin_resists: #hull_kin_resists,
                hull_exp_resists: #hull_exp_resists,
                armor_hp: #armor_hp,
                armor_em_resists: #armor_em_resists,
                armor_therm_resists: #armor_therm_resists,
                armor_kin_resists: #armor_kin_resists,
                armor_exp_resists: #armor_exp_resists,
                shield_hp: #shield_hp,
                shield_em_resists: #shield_em_resists,
                shield_therm_resists: #shield_therm_resists,
                shield_kin_resists: #shield_kin_resists,
                shield_exp_resists: #shield_exp_resists,
                sig_radius: #sig_radius,
            }
        };
        tok.to_tokens(tokens)
    }
}
