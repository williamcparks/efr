use crate::xml::{Element, Instr, NsBlock};

impl Element {
    pub fn print(&self, ns_block: Option<&NsBlock>) -> Vec<Instr> {
        let mut out = vec![Instr::Const(b"<".to_vec()), self.tag.print()];

        if let Some(ns_block) = ns_block {
            for ns in self.namespaces_used() {
                let uri = match ns_block.map.get(ns) {
                    Some(some) => some,
                    None => continue,
                };

                let bytes = format!(" xmlns:{}=\"{}\"", ns, uri.value());
                out.push(Instr::Const(bytes.into_bytes()));
            }
        }

        out.push(Instr::Const(b">".to_vec()));

        for child in self.children.iter() {
            out.extend(child.print());
        }

        out.push(Instr::Const(b"</".to_vec()));
        out.push(self.tag.print());
        out.push(Instr::Const(b">".to_vec()));

        out
    }
}
