use crate::xml::{Attr, AttrValue, Instr};

impl Attr {
    pub fn print(&self) -> Vec<Instr> {
        let mut out = vec![Instr::Const(b" ".to_vec()), self.tag.print()];

        out.push(Instr::Const(b"=\"".to_vec()));
        out.push(self.value.print());
        out.push(Instr::Const(b"\"".to_vec()));

        out
    }
}

impl AttrValue {
    pub fn print(&self) -> Instr {
        match self {
            Self::Const(lit) => Instr::Const(lit.value().into_bytes()),
            Self::Dynamic(el) => Instr::Dynamic(el.clone()),
        }
    }
}
