use std::collections::HashMap;

use syn::{
    Attribute, Error, Generics, Ident, LitStr, Meta, Token, Type, braced, parse::Parse,
    spanned::Spanned,
};

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
    pub map: HashMap<Ident, NsDecl>,
}

impl Parse for NsBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::custom_keyword!(ns);
        syn::custom_keyword!(inherit);

        input.parse::<Token![@]>()?;
        input.parse::<ns>()?;

        let section;
        braced!(section in input);

        let mut map = HashMap::new();

        while !section.is_empty() {
            let subelement = match section.peek(Token![#]) {
                false => false,
                true => {
                    NsDecl::parse_subelement_attr(&section)?;
                    true
                }
            };

            let key: Ident = section.parse()?;
            if subelement {
                section.parse::<Token![;]>()?;
                map.insert(key, NsDecl::SubElement);
                continue;
            }

            section.parse::<Token![=]>()?;
            let uri: LitStr = section.parse()?;
            section.parse::<Token![;]>()?;

            if map.contains_key(&key) {
                return Err(Error::new(key.span(), "duplicate namespace key"));
            }
            map.insert(key, NsDecl::Uri(uri));
        }

        Ok(Self { map })
    }
}

pub enum NsDecl {
    Uri(LitStr),
    SubElement,
}

impl NsDecl {
    pub fn parse_subelement_attr(input: syn::parse::ParseStream) -> syn::Result<()> {
        let attrs = Attribute::parse_outer(input)?;
        match attrs.as_slice() {
            [] => Err(Error::new(input.span(), "Expected #[subelement]")),
            [attr] => {
                if !attr.path().is_ident("subelement") {
                    return Err(Error::new(attr.span(), "Expected #[subelement]"));
                }
                if let Meta::Path(_) = attr.meta {
                    return Ok(());
                }

                Err(Error::new(
                    attr.span(),
                    "#[subelement] does not taking any arguments",
                ))
            }
            [_, other, ..] => Err(Error::new(
                other.span(),
                "Expected exactly one #[subelement] Attribute",
            )),
        }
    }
}
