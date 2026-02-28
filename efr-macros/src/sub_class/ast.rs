use std::collections::HashMap;

use syn::{DataEnum, Error, Fields, Ident, Result};

use crate::sub_class::{container_attr::ContainerAttr, variant_attr::VariantAttr};

pub struct Ast {
    pub map: HashMap<Ident, Vec<Ident>>,
}

impl Ast {
    pub fn try_new(data: DataEnum, container_attr: ContainerAttr) -> Result<Self> {
        let mut map = HashMap::<Ident, Vec<Ident>>::new();

        for variant in data.variants.into_iter() {
            if !matches!(variant.fields, Fields::Unit) {
                return Err(Error::new(
                    variant.ident.span(),
                    "Only Unit Variants Are Allowed With #[subclass]",
                ));
            }

            let variant_attr =
                VariantAttr::from_attrs(&variant.attrs, &container_attr, variant.ident.span())?;

            if let Some(prev) = map.get_mut(&variant_attr.subclass) {
                prev.push(variant.ident);
                continue;
            }

            map.insert(variant_attr.subclass, vec![variant.ident]);
        }

        Ok(Self { map })
    }
}
