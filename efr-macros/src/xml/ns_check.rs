use std::collections::HashSet;

use syn::Ident;

use crate::xml::{Child, Element, NsBlock, input::NsDecl};

impl Element {
    pub fn namespaces_used(&self) -> HashSet<&Ident> {
        let mut set = HashSet::new();

        if let Some(ns) = self.tag.prefix.as_ref() {
            set.insert(ns);
        }
        for attr in self.attrs.iter() {
            if let Some(ns) = attr.tag.prefix.as_ref() {
                set.insert(ns);
            }
        }
        for child in self.children.iter() {
            if let Child::Element(child) = child {
                set.extend(child.namespaces_used());
            }
        }

        set
    }
}

impl NsBlock {
    pub fn namespaces_declared(&self) -> HashSet<&Ident> {
        self.map.keys().collect()
    }

    pub fn subelement_namespaces(&self) -> HashSet<&Ident> {
        self.map
            .iter()
            .filter_map(|(k, v)| match v {
                NsDecl::SubElement => Some(k),
                _ => None,
            })
            .collect()
    }
}
