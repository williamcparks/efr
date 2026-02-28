use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::sub_class::ast::Ast;

impl Ast {
    pub fn print(&self) -> TokenStream {
        let funcs = self
            .map
            .iter()
            .map(|v| Self::print_one(v.0, v.1.as_slice()));

        quote! {
            #(#funcs)*
        }
    }

    pub fn print_one(subclass: &Ident, variants: &[Ident]) -> TokenStream {
        quote! {
            pub fn #subclass() -> &'static [Self] {
                &[
                    #(
                        Self::#variants
                    ),*
                ]
            }
        }
    }
}
