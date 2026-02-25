use proc_macro2::TokenStream;
use syn::{LitStr, braced, parenthesized, parse::Parse, token::Paren};

use crate::xml::Element;

pub enum Child {
    Const(LitStr),
    Dynamic(TokenStream),
    Element(Element),
}

impl Parse for Child {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Const(input.parse()?))
        } else if input.peek(Paren) {
            let section;
            parenthesized!(section in input);
            Ok(Self::Dynamic(section.parse()?))
        } else {
            Ok(Self::Element(input.parse()?))
        }
    }
}

impl Child {
    pub fn parse_children(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        let section;
        braced!(section in input);

        let mut children = Vec::new();
        while !section.is_empty() {
            children.push(section.parse()?);
        }

        Ok(children)
    }
}
