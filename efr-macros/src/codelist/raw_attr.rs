use syn::{
    Attribute, Error, LitStr, Result, Token, custom_keyword, parse::Parse, punctuated::Punctuated,
};

custom_keyword!(optional);

#[derive(Default)]
pub struct RawAttr {
    pub optional: Option<optional>,
    pub rename: Option<LitStr>,
}

impl RawAttr {
    pub fn try_new_opt(attrs: &[Attribute]) -> Result<Option<Self>> {
        let mut res: Option<Self> = None;

        for attr in attrs {
            if !attr.path().is_ident("codelist") {
                continue;
            }
            match res {
                Some(prev) => res = Some(prev.merge(attr.parse_args()?)?),
                None => {
                    res = Some(attr.parse_args()?);
                }
            }
        }

        Ok(res)
    }
}

impl Parse for RawAttr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let list = Punctuated::<Self, Token![,]>::parse_terminated_with(input, Self::parse_one)?;

        let mut res = Self::default();

        for value in list {
            res = res.merge(value)?;
        }

        Ok(res)
    }
}

impl RawAttr {
    fn parse_one(input: syn::parse::ParseStream) -> Result<Self> {
        match input.peek(LitStr) {
            true => Ok(Self {
                optional: None,
                rename: Some(input.parse()?),
            }),
            false => Ok(Self {
                optional: Some(input.parse()?),
                rename: None,
            }),
        }
    }

    fn merge(self, other: Self) -> Result<Self> {
        let opt = match (self.optional, other.optional) {
            (Some(opt), Some(_)) => {
                return Err(Error::new(opt.span, "`optional` Already Assigned"));
            }
            (Some(opt), None) | (None, Some(opt)) => Some(opt),
            (None, None) => None,
        };

        let rename = match (self.rename, other.rename) {
            (Some(some), Some(_)) => {
                return Err(Error::new(some.span(), "Field Is Already Renamed"));
            }
            (Some(some), None) | (None, Some(some)) => Some(some),
            (None, None) => None,
        };

        Ok(Self {
            optional: opt,
            rename,
        })
    }
}
