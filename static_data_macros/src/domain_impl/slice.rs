use proc_macro2::TokenStream;
use quote::ToTokens;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct SliceWrapper<T>(Box<[T]>);

impl<T> SliceWrapper<T> {
    pub fn new(v: Box<[T]>) -> Self {
        Self(v)
    }
}

impl<T> ToTokens for SliceWrapper<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner: String = self
            .0
            .iter()
            .map(|x| {
                let lok = quote::quote! {
                    #x
                };
                lok.to_string()
            })
            .collect::<Vec<String>>()
            .join(",");
        tokens.extend(TokenStream::from_str(format!("[{}]", inner).as_str()).unwrap())
    }
}
