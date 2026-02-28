use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Result};

use crate::sub_class::{ast::Ast, container_attr::ContainerAttr};

pub fn handler(input: DeriveInput) -> Result<TokenStream> {
    let msg = "Cannot #[derive(SubClass)] On Non-Enum";
    let data = match input.data {
        Data::Struct(s) => return Err(Error::new(s.struct_token.span, msg)),
        Data::Union(s) => return Err(Error::new(s.union_token.span, msg)),
        Data::Enum(e) => e,
    };

    let container_attr = ContainerAttr::from_attrs(input.attrs.as_slice(), input.ident.span())?;

    let ast = Ast::try_new(data, container_attr)?;
    let funcs = ast.print();

    let (impl_gen, type_gen, where_cl) = input.generics.split_for_impl();
    let ident = input.ident;

    Ok(quote! {
        #[automatically_derived]
        impl #impl_gen #ident #type_gen #where_cl {
            #funcs
        }
    })
}
