use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Result};

use crate::xml::{Input, Instr};

pub fn handler(input: Input) -> Result<TokenStream> {
    let impl_token = input.impl_block.impl_token;
    let impl_gen = input.impl_block.impl_gen;
    let for_token = input.impl_block.for_token;
    let ty = input.impl_block.ty;
    let ty_generics = input.impl_block.ty_generics;

    let declared_ns = input.ns_block.namespaces_declared();
    let mut used_ns = input.root.namespaces_used();
    used_ns.extend(input.ns_block.subelement_namespaces());

    if let Some(declared_not_used) = declared_ns.difference(&used_ns).next() {
        return Err(Error::new(
            declared_not_used.span(),
            "namespace declared but not used",
        ));
    } else if let Some(used_not_declared) = used_ns.difference(&declared_ns).next() {
        return Err(Error::new(used_not_declared.span(), "undeclared namespace"));
    }

    let instrs = Instr::optimize(input.root.print(Some(&input.ns_block)));
    let len_expr = instrs.iter().map(Instr::len_expr);
    let instrs = instrs.iter().map(Instr::print);

    Ok(quote! {
        #[automatically_derived]
        #impl_token #impl_gen crate::api::Xml #for_token #ty #ty_generics {
            fn xml(&self, xml: &mut ::std::vec::Vec<u8>) {
                #(#instrs)*
            }

            fn len(&self) -> usize {
                #(#len_expr)+*
            }
        }
    })
}
