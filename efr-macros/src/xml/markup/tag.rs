use syn::{Ident, Token, parse::Parse};

pub struct Tag {
    pub prefix: Option<Ident>,
    pub tag: Ident,
}

impl Parse for Tag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            return Ok(Self {
                prefix: Some(ident),
                tag: input.parse()?,
            });
        }

        Ok(Self {
            prefix: None,
            tag: ident,
        })
    }
}
