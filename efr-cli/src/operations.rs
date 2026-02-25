use efr::api::{Environment, Metadata, State};
use strum::{Display, VariantArray};

#[derive(Clone, Display, VariantArray)]
pub enum Operations {
    #[strum(to_string = "Authenticate User")]
    AuthenticateUser,

    #[strum(to_string = "Get Case List")]
    GetCaseList,

    #[strum(to_string = "Get Case")]
    GetCase,
}

pub mod authenticate_user;
pub mod error;
pub mod get_case;
pub mod get_case_list;

pub const METADATA: Metadata = Metadata {
    state: State::Texas,
    environment: Environment::Stage,
};
