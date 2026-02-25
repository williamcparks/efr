use crate::xml::{Child, Instr};

impl Child {
    pub fn print(&self) -> Vec<Instr> {
        match self {
            Self::Const(lit) => vec![Instr::Const(lit.value().into_bytes())],
            Self::Dynamic(el) => vec![Instr::Dynamic(el.clone())],
            Self::Element(el) => el.print(None),
        }
    }
}
