use std::iter::once;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

use crate::metadata::Input;

impl<'a> Input<'a> {
    pub fn print_try_new(&self) -> TokenStream {
        let state_arms = self
            .states
            .iter()
            .map(|state| {
                let abbr = LitStr::new(state.abbr, Span::call_site());
                let id = state.state_ident();

                quote! {
                    #abbr => ::core::result::Result::Ok(Self::#id)
                }
            })
            .chain(once(quote! {
                _ => ::core::result::Result::Err(StateError)
            }));

        let mut state_msg = "Available States: ".to_owned();
        for state in self.states.iter() {
            if !state_msg.ends_with(' ') {
                state_msg.push(',');
                state_msg.push(' ');
            }
            state_msg.push_str(state.abbr);
        }
        let state_msg = LitStr::new(state_msg.as_str(), Span::call_site());

        quote! {
            #[derive(Clone, Debug, ::thiserror::Error)]
            #[error(#state_msg)]
            pub struct StateError;

            impl State {
                pub fn try_new(state: &str) -> ::core::result::Result<Self, StateError> {
                    match state {
                        #(#state_arms),*
                    }
                }
            }
        }
    }
}
