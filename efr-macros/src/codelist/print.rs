use proc_macro2::TokenStream;
use quote::quote;
use syn::LitStr;

use crate::codelist::{Ast, AstField};

impl AstField {
    pub fn field_init(&self) -> TokenStream {
        let ident = &self.field;
        quote! {
            let mut #ident = ::std::borrow::Cow::Borrowed("");
        }
    }

    pub fn empty_field_check(&self, ast: &Ast) -> TokenStream {
        if self.optional {
            return TokenStream::new();
        }
        let ident = &self.field;

        let msg = format!(
            "Column `{}` Was Not Provided For {} List",
            self.column_ref.value(),
            ast.name.value(),
        );
        let msg = LitStr::new(msg.as_str(), ident.span());

        quote! {
            if #ident.is_empty() {
                return ::core::result::Result::Err(
                    crate::codes_service::EfrCodesError::Xml(
                        ::quick_xml::DeError::Custom(
                            #msg.to_owned()
                        )
                    )
                );
            }
        }
    }
}
