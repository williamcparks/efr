use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::metadata::{Input, State};

impl<'a> Input<'a> {
    pub fn print(&self) -> TokenStream {
        let states = self.states.iter().map(|state| state.state_ident());
        let user_service_arms = self.arms("efm/EFMUserService.svc");
        let firm_service_arms = self.arms("efm/EFMFirmService.svc");
        let court_record_service_arms = self.arms("efm/v5/CourtRecordService.svc");
        let court_policy_service_arms = self.arms("efm/v5/CourtPolicyService.svc");
        let service_arms = self.arms("efm/v5/Service.svc");
        let filing_assembly_service_arms = self.arms("efm/v5/FilingAssemblyService.svc");
        let filing_review_service_arms = self.arms("efm/v5/FilingReviewService.svc");
        let location_code_arms = self.arms("CodeService/codes/location");

        quote! {
            use serde::{Deserialize, Serialize};

            #[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
            pub struct Metadata {
                pub state: State,
                pub environment: Environment,
            }

            #[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
            pub enum State {
                #(#states),*
            }

            #[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
            pub enum Environment {
                Stage,
                Production,
            }

            impl Metadata {
                pub const fn user_service_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#user_service_arms)*
                    }
                }

                pub const fn firm_service_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#firm_service_arms)*
                    }
                }

                pub const fn court_record_service_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#court_record_service_arms)*
                    }
                }

                pub const fn court_policy_service_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#court_policy_service_arms)*
                    }
                }

                pub const fn service_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#service_arms)*
                    }
                }

                pub const fn filing_assembly_service_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#filing_assembly_service_arms)*
                    }
                }

                pub const fn filing_review_service(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#filing_review_service_arms)*
                    }
                }

                pub const fn location_codes_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#location_code_arms)*
                    }
                }
            }
        }
    }

    fn arms(&self, url: &str) -> impl Iterator<Item = TokenStream> {
        self.states.iter().map(|state| state.arm(url))
    }
}

impl<'a> State<'a> {
    fn arm(&self, url: &str) -> TokenStream {
        let ident = self.state_ident();
        let stage = format!("{}/{url}", self.stage_url);
        let prod = format!("{}/{url}", self.prod_url);

        quote! {
            (State::#ident, Environment::Stage) => #stage,
            (State::#ident, Environment::Production) => #prod,
        }
    }

    fn state_ident(&self) -> Ident {
        Ident::new(self.state, Span::call_site())
    }
}
