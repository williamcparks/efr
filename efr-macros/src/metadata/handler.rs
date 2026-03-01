use std::path::PathBuf;

use proc_macro2::TokenStream;
use syn::{Result, parse::Nothing, spanned::Spanned};

use crate::metadata::Input;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn handler(input: Nothing) -> Result<TokenStream> {
    let mut metadata_json_path = PathBuf::from(MANIFEST_DIR);
    metadata_json_path.push("metadata.json");

    let contents = Input::try_read(metadata_json_path.as_path(), input.span())?;
    let input = Input::try_parse(
        contents.as_str(),
        metadata_json_path.as_path(),
        input.span(),
    )?;

    Ok(input.print())
}
