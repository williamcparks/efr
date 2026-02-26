use syn::parse::Parse;

use crate::xml::{Attr, Child, Tag};

pub struct Element {
    pub tag: Tag,
    pub attrs: Vec<Attr>,
    pub children: Vec<Child>,
}

impl Parse for Element {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            tag: input.parse()?,
            attrs: Attr::parse_many(input)?,
            children: Child::parse_children(input)?,
        })
    }
}
