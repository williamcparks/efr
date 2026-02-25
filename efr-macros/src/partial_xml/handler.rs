use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::LitByteStr;

use crate::partial_xml::{Input, input::XmlToken};

pub fn handler(input: Input) -> TokenStream {
    let mut bytes = Vec::new();

    let mut tokens = Vec::<XmlToken>::new();
    for token in input.tokens {
        let requires_space = token.requires_space(tokens.last());

        if requires_space {
            tokens.push(XmlToken::Space);
        }

        tokens.push(token);
    }

    for token in tokens {
        token.print(&mut bytes);
    }

    let lit_bytes = LitByteStr::new(bytes.as_slice(), Span::call_site());
    lit_bytes.into_token_stream()
}
