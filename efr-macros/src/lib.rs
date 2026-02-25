use proc_macro::TokenStream;
use syn::parse_macro_input;

mod partial_xml;
mod xml;

#[proc_macro]
pub fn partial_xml(input: TokenStream) -> TokenStream {
    partial_xml::handler(parse_macro_input!(input)).into()
}

#[proc_macro]
pub fn xml(input: TokenStream) -> TokenStream {
    match xml::handler(parse_macro_input!(input)) {
        Ok(ok) => ok,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
