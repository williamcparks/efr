use crate::xml::Instr;

impl Instr {
    pub fn optimize(list: Vec<Self>) -> Vec<Self> {
        let mut out = Vec::new();

        for instr in list {
            match (instr, out.last_mut()) {
                (Instr::Const(new_bytes), Some(Instr::Const(prev_bytes))) => {
                    prev_bytes.extend_from_slice(new_bytes.as_slice())
                }
                (instr, _) => out.push(instr),
            };
        }

        out
    }
}
