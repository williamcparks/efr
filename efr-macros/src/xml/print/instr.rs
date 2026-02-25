use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{LitByteStr, LitInt};

pub enum Instr {
    Dynamic(TokenStream),
    Const(Vec<u8>),
}

impl Instr {
    pub fn len_expr(&self) -> TokenStream {
        match self {
            Self::Dynamic(el) => {
                quote! {
                    crate::api::Xml::len(#el)
                }
            }
            Self::Const(bytes) => {
                let len = bytes.len().to_string();
                let len = LitInt::new(len.as_str(), Span::call_site());
                len.into_token_stream()
            }
        }
    }

    pub fn print(&self) -> TokenStream {
        match self {
            Self::Dynamic(el) => {
                quote! {
                    crate::api::Xml::xml(#el, xml);
                }
            }
            Self::Const(bytes) => {
                let byte_str = LitByteStr::new(bytes.as_slice(), Span::call_site());
                quote! {
                    xml.extend_from_slice(#byte_str);
                }
            }
        }
    }
}
