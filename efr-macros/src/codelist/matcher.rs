use std::iter::once;

use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::codelist::{Ast, AstField};

impl Ast {
    pub fn matchers(&self) -> TokenStream {
        let msg = format!(
            "Unknown Column `{{_fallback_binding}}` In {} List",
            self.name.value()
        );
        let msg = LitStr::new(msg.as_str(), self.name.span());

        let mut list = Vec::new();

        let len = self.fields.len();
        for _ in 0..len {
            let arms = self.fields.iter().map(AstField::arm).chain(once(quote! {
                _fallback_binding => return ::core::result::Result::Err(
                    crate::codes_service::EfrCodesError::Xml(
                        ::quick_xml::DeError::Custom(
                            ::std::format!(#msg),
                        )
                    )
                )
            }));

            list.push(quote! {
                let _col = pull.col()?;
                match _col.column_ref.as_str() {
                    #(#arms),*
                }
            });
        }

        quote! {
            #(#list)*
        }
    }
}

impl AstField {
    fn arm(&self) -> TokenStream {
        let lit = &self.column_ref;
        let id = &self.field;

        quote! {
            #lit => {
                #id = _col.value;
            }
        }
    }
}
