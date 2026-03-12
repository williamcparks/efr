use syn::{Attribute, DataStruct, Error, Fields, Ident, LitStr, Result};

use crate::codelist::Attr;

pub struct Ast {
    pub name: LitStr,
    pub fields: Vec<AstField>,
}

pub struct AstField {
    pub column_ref: LitStr,
    pub optional: bool,
    pub field: Ident,
}

impl Ast {
    pub fn try_new(data: DataStruct, name: LitStr) -> Result<Self> {
        if let Fields::Unit = &data.fields {
            return Err(Error::new(
                data.struct_token.span,
                "Unit Structs Cannot #[derive(CodeList)]",
            ));
        }

        let mut fields = Vec::new();
        for field in data.fields {
            let ident = match field.ident {
                Some(some) => some,
                None => {
                    return Err(Error::new(
                        data.struct_token.span,
                        "Unnamed Structs Cannot #[derive(CodeList)]",
                    ));
                }
            };

            fields.push(AstField::try_new(ident, field.attrs.as_slice())?)
        }

        Ok(Self { name, fields })
    }
}

impl AstField {
    pub fn try_new(ident: Ident, attrs: &[Attribute]) -> Result<Self> {
        let attr = Attr::try_new(attrs, &ident)?;

        Ok(Self {
            column_ref: attr.rename,
            optional: attr.optional,
            field: ident,
        })
    }
}
