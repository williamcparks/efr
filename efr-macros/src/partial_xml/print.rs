use crate::partial_xml::input::XmlToken;

impl XmlToken {
    pub fn requires_space(&self, prev: Option<&Self>) -> bool {
        matches!(
            (self, prev),
            (Self::Ident(_), Some(Self::Ident(_) | Self::LitStr(_)))
        )
    }

    pub fn print(self, out: &mut Vec<u8>) {
        match self {
            Self::OpenAngle => out.push(b'<'),
            Self::CloseAngle => out.push(b'>'),
            Self::Colon => out.push(b':'),
            Self::Slash => out.push(b'/'),
            Self::Equal => out.push(b'='),
            Self::Ident(ident) => {
                out.extend(ident.to_string().as_bytes());
            }
            Self::LitStr(lit_str) => {
                out.push(b'"');
                out.extend(lit_str.as_bytes());
                out.push(b'"');
            }
            Self::PartialLitStr(lit_str) => {
                out.push(b'"');
                out.extend(lit_str.as_bytes());
            }
            Self::Space => out.push(b' '),
        }
    }
}
