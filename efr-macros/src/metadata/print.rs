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

        // System-wide (static)
        let location_arms = self.arms("CodeService/codes/location/");
        let version_arms = self.arms("CodeService/codes/versions/");
        let error_arms = self.arms("CodeService/codes/error/");
        let country_arms = self.arms("CodeService/codes/country/");
        let state_arms = self.arms("CodeService/codes/state/");
        let filing_status_arms = self.arms("CodeService/codes/filingstatus/");
        let data_field_config_arms = self.arms("CodeService/codes/datafield/");
        let language_arms = self.arms("CodeService/codes/language/");

        // Court-specific (jurisdiction, use format!)
        let answer_arms = self.arms("CodeService/codes/answer/");
        let appellate_lower_courts_arms = self.arms("CodeService/codes/appellatelowercourt/");
        let case_category_arms = self.arms("CodeService/codes/casecategory/");
        let case_sub_type_arms = self.arms("CodeService/codes/casesubtype/");
        let case_type_arms = self.arms("CodeService/codes/casetype/");
        let cross_reference_arms = self.arms("CodeService/codes/crossreference/");
        let damage_amount_arms = self.arms("CodeService/codes/damageamount/");
        let disclaimer_requirement_arms = self.arms("CodeService/codes/disclaimerrequirement/");
        let document_type_arms = self.arms("CodeService/codes/documenttype/");
        let filer_type_arms = self.arms("CodeService/codes/filertype/");
        let file_type_arms = self.arms("CodeService/codes/filetype/");
        let filing_arms = self.arms("CodeService/codes/filing/");
        let filing_component_arms = self.arms("CodeService/codes/filingcomponent/");
        let hearing_location_arms = self.arms("CodeService/codes/hearinglocation/");
        let judicial_officer_arms = self.arms("CodeService/codes/judicialofficer/");
        let motion_type_arms = self.arms("CodeService/codes/motiontype/");
        let name_suffix_arms = self.arms("CodeService/codes/namesuffix/");
        let notification_agency_arms = self.arms("CodeService/codes/notificationagency/");
        let optional_services_arms = self.arms("CodeService/codes/optionalservice/");
        let party_type_arms = self.arms("CodeService/codes/partytype/");
        let procedure_remedy_arms = self.arms("CodeService/codes/procedureremedy/");
        let question_arms = self.arms("CodeService/codes/question/");
        let refund_reason_arms = self.arms("CodeService/codes/refundreason/");
        let rep_cap_arms = self.arms("CodeService/codes/repcap/");
        let service_provider_arms = self.arms("CodeService/codes/serviceprovider/");
        let service_type_arms = self.arms("CodeService/codes/servicetype/");

        // Criminal initiation (jurisdiction, use format!)
        let arrest_location_arms = self.arms("CodeService/codes/arrestlocation/");
        let bond_arms = self.arms("CodeService/codes/bond/");
        let charge_phase_arms = self.arms("CodeService/codes/chargephase/");
        let citation_jurisdiction_arms = self.arms("CodeService/codes/citationjurisdiction/");
        let degree_arms = self.arms("CodeService/codes/degree/");
        let driver_license_type_arms = self.arms("CodeService/codes/driverlicensetype/");
        let ethnicity_arms = self.arms("CodeService/codes/ethnicity/");
        let eye_color_arms = self.arms("CodeService/codes/eyecolor/");
        let general_offense_arms = self.arms("CodeService/codes/generaloffense/");
        let hair_color_arms = self.arms("CodeService/codes/haircolor/");
        let law_enforcement_unit_arms = self.arms("CodeService/codes/lawenforcementunit/");
        let physical_feature_arms = self.arms("CodeService/codes/physicalfeature/");
        let race_arms = self.arms("CodeService/codes/race/");
        let statute_arms = self.arms("CodeService/codes/statute/");
        let statute_type_arms = self.arms("CodeService/codes/statutetype/");
        let vehicle_color_arms = self.arms("CodeService/codes/vehiclecolor/");
        let vehicle_make_arms = self.arms("CodeService/codes/vehiclemake/");
        let vehicle_type_arms = self.arms("CodeService/codes/vehicletype/");

        let try_new_impls = self.print_try_new();

        quote! {
            use serde::{Deserialize, Serialize};

            #[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
            pub struct Metadata {
                pub state: State,
                pub environment: Environment,
            }

            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
            pub enum State {
                #(#states),*
            }

            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
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


                   // ── System-wide ──────────────────────────────────────────────────────────
                pub const fn locations_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#location_arms)*
                    }
                }

                pub const fn version_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#version_arms)*
                    }
                }

                pub const fn error_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#error_arms)*
                    }
                }

                pub const fn country_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#country_arms)*
                    }
                }

                pub const fn state_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#state_arms)*
                    }
                }

                pub const fn filing_status_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#filing_status_arms)*
                    }
                }

                pub const fn data_field_config_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#data_field_config_arms)*
                    }
                }

                pub const fn language_url(&self) -> &'static str {
                    match (self.state, self.environment) {
                        #(#language_arms)*
                    }
                }

                // ── Court-specific ───────────────────────────────────────────────────────

                pub fn answer_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#answer_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn appellate_lower_courts_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#appellate_lower_courts_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn case_category_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#case_category_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn case_sub_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#case_sub_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn case_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#case_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn cross_reference_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#cross_reference_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn damage_amount_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#damage_amount_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn disclaimer_requirement_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#disclaimer_requirement_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn document_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#document_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn filer_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#filer_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn file_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#file_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn filing_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#filing_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn filing_component_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#filing_component_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn hearing_location_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#hearing_location_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn judicial_officer_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#judicial_officer_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn motion_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#motion_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn name_suffix_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#name_suffix_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn notification_agency_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#notification_agency_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn optional_services_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#optional_services_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn party_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#party_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn procedure_remedy_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#procedure_remedy_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn question_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#question_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn refund_reason_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#refund_reason_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn rep_cap_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#rep_cap_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn service_provider_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#service_provider_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn service_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#service_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                // ── Criminal initiation ──────────────────────────────────────────────────

                pub fn arrest_location_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#arrest_location_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn bond_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#bond_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn charge_phase_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#charge_phase_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn citation_jurisdiction_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#citation_jurisdiction_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn degree_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#degree_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn driver_license_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#driver_license_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn ethnicity_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#ethnicity_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn eye_color_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#eye_color_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn general_offense_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#general_offense_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn hair_color_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#hair_color_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn law_enforcement_unit_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#law_enforcement_unit_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn physical_feature_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#physical_feature_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn race_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#race_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn statute_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#statute_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn statute_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#statute_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn vehicle_color_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#vehicle_color_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn vehicle_make_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#vehicle_make_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }

                pub fn vehicle_type_url(&self, jurisdiction: &str) -> String {
                    let base = match (self.state, self.environment) { #(#vehicle_type_arms)* };
                    ::std::format!("{base}{jurisdiction}")
                }
            }

            #try_new_impls
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

    pub fn state_ident(&self) -> Ident {
        Ident::new(self.state, Span::call_site())
    }
}
