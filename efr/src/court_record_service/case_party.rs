use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CaseParty<'a> {
    x: &'a str,
}
