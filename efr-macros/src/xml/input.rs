use std::collections::HashMap;

use syn::{
    Attribute, Error, Generics, Ident, LitStr, Token, Type, braced, parse::Parse, spanned::Spanned,
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
            let ns_decl_attr = match section.peek(Token![#]) {
                false => None,
                true => Some(section.parse::<NsDeclAttr>()?),
            };

            let key: Ident = section.parse()?;
            match ns_decl_attr {
                Some(NsDeclAttr::SubElement) => {
                    section.parse::<Token![;]>()?;
                    map.insert(key, NsDecl::SubElement);
                    continue;
                }
                Some(NsDeclAttr::Parent) => {
                    section.parse::<Token![=]>()?;
                    let uri: LitStr = section.parse()?;
                    section.parse::<Token![;]>()?;

                    if map.contains_key(&key) {
                        return Err(Error::new(key.span(), "duplicate namespace key"));
                    }
                    map.insert(key, NsDecl::ParentUri(uri));
                }
                None => {
                    section.parse::<Token![=]>()?;
                    let uri: LitStr = section.parse()?;
                    section.parse::<Token![;]>()?;

                    if map.contains_key(&key) {
                        return Err(Error::new(key.span(), "duplicate namespace key"));
                    }
                    map.insert(key, NsDecl::Uri(uri));
                }
            }
        }

        Ok(Self { map })
    }
}

pub enum NsDecl {
    Uri(LitStr),
    ParentUri(LitStr),
    SubElement,
}

enum NsDeclAttr {
    Parent,
    SubElement,
}

impl Parse for NsDeclAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = Attribute::parse_outer(input)?;
        let attr = match attrs.as_slice() {
            [] => Err(Error::new(
                input.span(),
                "Expected #[subelement] or #[parent]",
            )),
            [attr] => Ok(attr),
            [_, other, ..] => Err(Error::new(
                other.span(),
                "Expected exactly one #[subelement] or #[parent] Attribute",
            )),
        }?;

        if attr.path().is_ident("subelement") {
            Ok(Self::SubElement)
        } else if attr.path().is_ident("parent") {
            Ok(Self::Parent)
        } else {
            Err(Error::new(
                attr.span(),
                "Expected #[subelement] or #[parent]",
            ))
        }
    }
}
