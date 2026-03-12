use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Result};

use crate::codelist::{Ast, AstField, Attr, lt};

pub fn handler(input: DeriveInput) -> Result<TokenStream> {
    let data = match input.data {
        Data::Struct(s) => s,
        Data::Enum(e) => {
            return Err(Error::new(
                e.enum_token.span,
                "Cannot #[derive(CodeList)] On `enum`",
            ));
        }
        Data::Union(u) => {
            return Err(Error::new(
                u.union_token.span,
                "Cannot #[derive(CodeList)] On `union`",
            ));
        }
    };

    let ast = Ast::try_new(data, Attr::container(input.attrs.as_slice(), &input.ident)?)?;

    let (cloned_generics, lt) = lt(input.generics.clone());
    let (impl_gen, _, _) = cloned_generics.split_for_impl();
    let (_, type_gen, where_cl) = input.generics.split_for_impl();
    let ident = input.ident;

    let field_inits = ast.fields.iter().map(AstField::field_init);
    let fields = ast.fields.iter().map(|ast| &ast.field);
    let empty_field_check = ast
        .fields
        .iter()
        .map(|field| AstField::empty_field_check(field, &ast));
    let matchers = ast.matchers();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_gen crate::codes_service::utils::CodeRow<#lt> for #ident #type_gen #where_cl {
            fn code_row(ns_reader: &mut ::quick_xml::NsReader<&'a [u8]>) -> ::core::result::Result<::core::option::Option<Self>, crate::codes_service::EfrCodesError> {
                let mut pull = crate::codes_service::utils::Pull(ns_reader);
                if !pull.open_row()? {
                    return ::core::result::Result::Ok(
                        ::core::option::Option::None
                    );
                }

                #(#field_inits)*

                #matchers

                pull.0.read_to_end(
                    ::quick_xml::name::QName(b"Row")
                )?;

                #(#empty_field_check)*

                ::core::result::Result::Ok(
                    ::core::option::Option::Some(
                        Self {
                            #(#fields),*
                        }
                    )
                )
            }
        }
    })
}
