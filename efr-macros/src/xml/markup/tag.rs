use syn::{Ident, Token, parse::Parse};

pub struct Tag {
    pub prefix: Option<Ident>,
    pub tag: Ident,
}

impl Parse for Tag {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let raw = parse_tag(input)?;

        Ok(Self {
            prefix: raw.prefix.map(modify_ident),
            tag: modify_ident(raw.tag),
        })
    }
}

fn parse_tag(input: syn::parse::ParseStream) -> syn::Result<Tag> {
    let ident: Ident = input.parse()?;
    if input.peek(Token![:]) {
        input.parse::<Token![:]>()?;
        return Ok(Tag {
            prefix: Some(ident),
            tag: input.parse()?,
        });
    }

    Ok(Tag {
        prefix: None,
        tag: ident,
    })
}

fn modify_ident(ident: Ident) -> Ident {
    let raw = ident.to_string();
    Ident::new(raw.trim_start_matches("r#"), ident.span())
}
