use syn::{Attribute, Error, Ident, LitStr, Result, spanned::Spanned};

use crate::codelist::RawAttr;

pub struct Attr {
    pub optional: bool,
    pub rename: LitStr,
}

impl Attr {
    pub fn container(attrs: &[Attribute], ident: &Ident) -> Result<LitStr> {
        let mut res = None;

        for attr in attrs {
            if !attr.path().is_ident("codelist") {
                continue;
            }
            if res.is_some() {
                return Err(Error::new(
                    attr.span(),
                    "#[codelist(`...`)] Attribute Already Defined",
                ));
            }
            res = Some(attr.parse_args()?);
        }

        match res {
            Some(some) => Ok(some),
            None => Err(Error::new(
                ident.span(),
                "Missing #[codelist(`...`)] Attribute On Struct",
            )),
        }
    }

    pub fn try_new(attrs: &[Attribute], ident: &Ident) -> Result<Self> {
        match RawAttr::try_new_opt(attrs)? {
            Some(some) => Ok(Self::new(some, ident)),
            None => Ok(Self {
                optional: false,
                rename: LitStr::new(ident.to_string().as_str(), ident.span()),
            }),
        }
    }

    fn new(raw_attr: RawAttr, ident: &Ident) -> Self {
        let rename = match raw_attr.rename {
            Some(some) => some,
            None => LitStr::new(ident.to_string().as_str(), ident.span()),
        };

        Self {
            optional: raw_attr.optional.is_some(),
            rename,
        }
    }
}
