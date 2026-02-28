use std::fmt::Display;

use proc_macro2::Span;
use syn::{Attribute, Error, Ident, Result, spanned::Spanned};

use crate::sub_class::container_attr::ContainerAttr;

pub struct VariantAttr {
    pub subclass: Ident,
}

impl VariantAttr {
    pub fn from_attrs(
        attrs: &[Attribute],
        container_attr: &ContainerAttr,
        span: Span,
    ) -> Result<Self> {
        let mut res: Option<Self> = None;

        for attr in attrs {
            if !attr.path().is_ident("subclass") {
                continue;
            }
            if res.is_some() {
                return Err(Error::new(attr.span(), "Duplicate #[subclass] Attribute"));
            }
            let id: Ident = attr.parse_args()?;
            if !container_attr.sub_classes.contains(&id) {
                return Err(Error::new(id.span(), UnknownSubClass(container_attr)));
            }
            res = Some(Self { subclass: id });
        }

        match res {
            Some(some) => Ok(some),
            None => Err(Error::new(span, NoSubClass(container_attr))),
        }
    }
}

struct NoSubClass<'a>(&'a ContainerAttr);

impl<'a> Display for NoSubClass<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No #[subclass(...)] Assigned, examples:")?;

        let mut iter = self.0.sub_classes.iter();
        if let Some(first) = iter.next() {
            write!(f, " `{first}`")?;
        }
        for subclass in iter {
            write!(f, ", {subclass}")?;
        }
        Ok(())
    }
}

struct UnknownSubClass<'a>(&'a ContainerAttr);

impl<'a> Display for UnknownSubClass<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown #[subclass(...)] value, examples:")?;

        let mut iter = self.0.sub_classes.iter();
        if let Some(first) = iter.next() {
            write!(f, " `{first}`")?;
        }
        for subclass in iter {
            write!(f, ", {subclass}")?;
        }
        Ok(())
    }
}
