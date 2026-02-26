use proc_macro2::TokenStream;
use syn::{LitStr, parenthesized, parse::Parse, token::Paren};

pub enum AttrValue {
    Const(LitStr),
    Dynamic(TokenStream),
}

impl Parse for AttrValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.peek(Paren) {
            true => {
                let section;
                parenthesized!(section in input);
                Ok(Self::Dynamic(section.parse()?))
            }
            false => Ok(Self::Const(input.parse()?)),
        }
    }
}
