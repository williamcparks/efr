use std::collections::HashMap;

use syn::{Error, Generics, Ident, LitStr, Token, Type, braced, parse::Parse};

use crate::xml::Element;

pub struct Input {
    pub impl_block: ImplBlock,
    pub ns_block: NsBlock,
    pub root: Element,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let impl_block = input.parse()?;

        let section;
        braced!(section in input);

        Ok(Self {
            impl_block,
            ns_block: section.parse()?,
            root: section.parse()?,
        })
    }
}

pub struct ImplBlock {
    pub impl_token: Token![impl],
    pub impl_gen: Generics,
    pub for_token: Token![for],
    pub ty: Type,
    pub ty_generics: Generics,
}

impl Parse for ImplBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            impl_token: input.parse()?,
            impl_gen: {
                let impl_gen = input.parse()?;
                syn::custom_keyword!(Xml);
                input.parse::<Xml>()?;
                impl_gen
            },
            for_token: input.parse()?,
            ty: input.parse()?,
            ty_generics: input.parse()?,
        })
    }
}

pub struct NsBlock {
    pub map: HashMap<Ident, LitStr>,
}

impl Parse for NsBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(ns);

        input.parse::<Token![@]>()?;
        input.parse::<ns>()?;
        let section;
        braced!(section in input);

        let mut map = HashMap::new();

        while !section.is_empty() {
            let key: Ident = section.parse()?;
            section.parse::<Token![=]>()?;
            let value: LitStr = section.parse()?;
            section.parse::<Token![;]>()?;

            if map.contains_key(&key) {
                return Err(Error::new(key.span(), "duplicate namespace key"));
            }
            map.insert(key, value);
        }

        Ok(Self { map })
    }
}
