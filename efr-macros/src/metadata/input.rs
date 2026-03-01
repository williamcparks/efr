use std::path::Path;

use proc_macro2::Span;
use serde::Deserialize;
use syn::{Error, Result};

#[derive(Deserialize)]
pub struct Input<'a> {
    #[serde(borrow)]
    pub states: Vec<State<'a>>,
}

#[derive(Deserialize)]
pub struct State<'a> {
    pub state: &'a str,
    // pub status: &'a str,
    pub stage_url: &'a str,
    pub prod_url: &'a str,
    // pub review_stage_url: &'a str,
    // pub toga_stage_client_key: &'a str,
    // pub toga_prod_client_key: Option<&'a str>,
}

impl<'a> Input<'a> {
    pub fn try_read(path: &Path, span: Span) -> Result<String> {
        match std::fs::read_to_string(path) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                span,
                format!("Failed To Read `{}` Due To: {err}", path.display()),
            )),
        }
    }

    pub fn try_parse(contents: &'a str, path: &Path, span: Span) -> Result<Self> {
        match serde_json::from_str(contents) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(
                span,
                format!("Failed To Parse `{}` Due To: {err}", path.display()),
            )),
        }
    }
}
