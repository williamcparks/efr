use crate::xml::{Instr, Tag};

impl Tag {
    pub fn print(&self) -> Instr {
        let mut bytes = Vec::new();

        if let Some(prefix) = self.prefix.as_ref() {
            bytes.extend_from_slice(prefix.to_string().as_bytes());
            bytes.push(b':');
        }
        bytes.extend_from_slice(self.tag.to_string().as_bytes());

        Instr::Const(bytes)
    }
}
