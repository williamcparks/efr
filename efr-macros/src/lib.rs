#[cfg(any(feature = "partial-xml", feature = "xml", feature = "sub-class"))]
use proc_macro::TokenStream;
#[cfg(any(feature = "partial-xml", feature = "xml", feature = "sub-class"))]
use syn::parse_macro_input;

#[cfg(feature = "partial-xml")]
mod partial_xml;
#[cfg(feature = "sub-class")]
mod sub_class;
#[cfg(feature = "xml")]
mod xml;

#[cfg(feature = "partial-xml")]
#[proc_macro]
pub fn partial_xml(input: TokenStream) -> TokenStream {
    partial_xml::handler(parse_macro_input!(input)).into()
}

#[cfg(feature = "xml")]
#[proc_macro]
pub fn xml(input: TokenStream) -> TokenStream {
    match xml::handler(parse_macro_input!(input)) {
        Ok(ok) => ok,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

#[cfg(feature = "sub-class")]
#[proc_macro_derive(SubClass, attributes(subclass))]
pub fn sub_class(input: TokenStream) -> TokenStream {
    match sub_class::handler(parse_macro_input!(input)) {
        Ok(ok) => ok,
        Err(err) => err.into_compile_error(),
    }
    .into()
}
