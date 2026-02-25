use syn::{Error, Ident, LitStr, Token, parse::Parse};

pub struct Input {
    pub tokens: Vec<XmlToken>,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut tokens = Vec::new();

        while !input.is_empty() {
            tokens.push(XmlToken::parse(input)?);
        }

        Ok(Self { tokens })
    }
}

pub enum XmlToken {
    OpenAngle,
    CloseAngle,
    Colon,
    Slash,
    Equal,

    Ident(String),
    LitStr(String),
    PartialLitStr(String),

    Space,
}

impl Parse for XmlToken {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            Ok(Self::OpenAngle)
        } else if input.peek(Token![>]) {
            input.parse::<Token![>]>()?;
            Ok(Self::CloseAngle)
        } else if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Ok(Self::Colon)
        } else if input.peek(Token![/]) {
            input.parse::<Token![/]>()?;
            Ok(Self::Slash)
        } else if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            Ok(Self::Equal)
        } else if input.peek(Ident) {
            Ok(Self::Ident(input.parse::<Ident>()?.to_string()))
        } else if input.peek(LitStr) {
            Ok(Self::LitStr(input.parse::<LitStr>()?.value()))
        } else if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(Self::PartialLitStr(input.parse::<LitStr>()?.value()))
        } else {
            Err(Error::new(input.span(), "Unknown Partial XML Token"))
        }
    }
}
