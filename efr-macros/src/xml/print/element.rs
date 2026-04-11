use crate::xml::{Element, Instr, NsBlock, input::NsDecl};

impl Element {
    pub fn print(&self, ns_block: Option<&NsBlock>) -> Vec<Instr> {
        let mut out = vec![Instr::Const(b"<".to_vec()), self.tag.print()];

        if let Some(ns_block) = ns_block {
            for (ns, decl) in ns_block.map.iter() {
                let uri = match decl {
                    NsDecl::Uri(uri) | NsDecl::ParentUri(uri) => uri,
                    NsDecl::SubElement => continue,
                };

                let bytes = format!(" xmlns:{}=\"{}\"", ns, uri.value());
                out.push(Instr::Const(bytes.into_bytes()));
            }
        }

        for attr in self.attrs.iter() {
            out.extend(attr.print());
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
