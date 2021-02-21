use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, Fields};

#[proc_macro_derive(Stat)]
pub fn stat_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    let a = impl_stat_macro(&ast);
    a
}

fn impl_stat_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let types = match ast.data.clone() {
        Data::Struct(x) => match x.fields {
            Fields::Named(x) => x
                .named
                .iter()
                .map(|x| {
                    (
                        x.ident.as_ref().unwrap().to_string(),
                        x.ty.to_token_stream().to_string(),
                    )
                })
                .collect::<Vec<(String, String)>>(),
            Fields::Unnamed(_) => panic!("cant derive Stat with unnamed fields"),
            Fields::Unit => panic!("cant derive Stat with unit fields"),
        },
        Data::Enum(_) => panic!("cant derive Stat enums"),
        Data::Union(_) => panic!("cant derive Stat on unions"),
    };
    let mod_name = format!("{}Modifications", name.to_string())
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let mod_fields = types
        .iter()
        .enumerate()
        .map(|(i, (name, _))| format!("pub {}: crate::stats::ModificationType<_{}>,", name, i))
        .collect::<String>()
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let new_arg_list = types
        .iter()
        .enumerate()
        .map(|(i, (name, _))| format!("{}: crate::stats::ModificationType<_{}>,", name, i))
        .collect::<String>()
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let self_arg_list = types
        .iter()
        .map(|(name, _)| format!("{},", name))
        .collect::<String>()
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let field_unwrap = format!(
        "let ({})",
        types
            .iter()
            .map(|(name, _)| name.to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap();
    let vec_input = format!(
        "({})",
        types
            .iter()
            .map(|_| "vec![]".to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap();
    let fold_func = format!(
        r"|({}), x| {{
            let {} {{ {} }} = x;
            {}
            ({})
        }}",
        types
            .iter()
            .map(|(name, _)| format!("mut {}_vec,", name))
            .collect::<String>(),
        mod_name.to_string(),
        types
            .iter()
            .map(|(name, _)| format!("{},", name))
            .collect::<String>(),
        types
            .iter()
            .map(|(name, _)| format!("{0}_vec.push({0});", name))
            .collect::<String>(),
        types
            .iter()
            .map(|(name, _)| format!("{}_vec,", name))
            .collect::<String>(),
    )
    .parse::<proc_macro2::TokenStream>()
    .unwrap();
    let calculation = types
        .iter()
        .map(|(name, _)| format!("r.{0} = calculate(r.{0}, {0});", name))
        .collect::<String>()
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let generic_list = types
        .iter()
        .enumerate()
        .map(|(i, _)| format!("_{},", i))
        .collect::<String>()
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let where_clause = types
        .iter()
        .enumerate()
        .map(|(i, (_, tt))| {
            format!(
                r"
        {0}: num_traits::cast::AsPrimitive<_{1}>,
        _{1}: num_traits::NumOps
        + PartialEq
        + PartialOrd
        + Clone
        + num_traits::Zero
        + num_traits::cast::AsPrimitive<{0}>,
        ",
                tt, i
            )
        })
        .collect::<String>()
        .parse::<proc_macro2::TokenStream>()
        .unwrap();
    let gen = quote! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct #mod_name<#generic_list> where #where_clause {
            #mod_fields
        }

        impl<#generic_list> #mod_name<#generic_list> where #where_clause {
            pub fn new(#new_arg_list) -> Self {
                Self {#self_arg_list}
            }
        }

        impl<#generic_list> Stat<#mod_name<#generic_list>> for #name where #where_clause {
            fn apply(&self, stat_mods: Vec<&#mod_name<#generic_list>>) -> Self {
                fn calculate<T, V>(base_val: T, mut additions: Vec<&crate::stats::ModificationType<V>>) -> T
                where
                    T: num_traits::cast::AsPrimitive<V>,
                    V: num_traits::cast::AsPrimitive<T>,
                    T: num_traits::NumOps + PartialEq + PartialOrd + Clone + num_traits::Zero,
                    V: num_traits::NumOps + PartialEq + PartialOrd + Clone + num_traits::Zero,
                {
                    additions.sort_by(|a,b|b.partial_cmp(a).unwrap());
                    additions.into_iter().fold(base_val, |acc, x| x.apply(acc))
                }
                let mut r = self.clone();
                #field_unwrap = stat_mods.into_iter().fold(#vec_input, #fold_func);
                #calculation
                r
            }
        }
    };
    gen.into()
}
