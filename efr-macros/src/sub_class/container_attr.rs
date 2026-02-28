use std::collections::HashSet;

use proc_macro2::Span;
use syn::{Attribute, Error, Ident, Result, Token, parse::Parse, spanned::Spanned};

pub struct ContainerAttr {
    pub sub_classes: HashSet<Ident>,
}

impl ContainerAttr {
    pub fn from_attrs(attrs: &[Attribute], span: Span) -> Result<Self> {
        let mut res: Option<Self> = None;
        for attr in attrs {
            if !attr.path().is_ident("subclass") {
                continue;
            }
            if res.is_some() {
                return Err(Error::new(attr.span(), "Duplicate #[subclass] Attribute"));
            }
            res = Some(attr.parse_args::<Self>()?);
        }

        match res {
            Some(some) => Ok(some),
            None => Err(Error::new(
                span,
                "#[derive(SubClass)] Requires A Container #[subclass(...)] Attribute",
            )),
        }
    }
}

impl Parse for ContainerAttr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut sub_classes = HashSet::new();

        let list = input.parse_terminated(Ident::parse, Token![,])?;
        for ident in list {
            if sub_classes.contains(&ident) {
                return Err(Error::new(ident.span(), "Subclass Already Declared"));
            }
            sub_classes.insert(ident);
        }

        Ok(ContainerAttr { sub_classes })
    }
}
