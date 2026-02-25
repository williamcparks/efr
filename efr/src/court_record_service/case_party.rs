use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CaseParty<'a> {
    pub first_name: &'a str,
    pub middle_name: &'a str,
    pub last_name: &'a str,
    pub business_name: &'a str,
}
