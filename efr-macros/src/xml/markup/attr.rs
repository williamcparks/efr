use syn::{Token, parse::Parse, token::Brace};

use crate::xml::{AttrValue, Tag};

pub struct Attr {
    pub tag: Tag,
    pub value: AttrValue,
}

impl Parse for Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let tag = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;

        Ok(Self { tag, value })
    }
}

impl Attr {
    pub fn parse_many(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        let mut attrs = Vec::new();
        while !input.peek(Brace) {
            attrs.push(input.parse()?);
        }

        Ok(attrs)
    }
}
