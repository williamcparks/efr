use syn::parse::Parse;

use crate::xml::{Child, Tag};

pub struct Element {
    pub tag: Tag,
    pub children: Vec<Child>,
}

impl Parse for Element {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            tag: input.parse()?,
            children: Child::parse_children(input)?,
        })
    }
}
