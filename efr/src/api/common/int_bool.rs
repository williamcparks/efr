use crate::api::Xml;

pub struct IntBool(pub bool);

impl Xml for IntBool {
    fn len(&self) -> usize {
        1
    }

    fn xml(&self, xml: &mut Vec<u8>) {
        let byte = match self.0 {
            false => b'0',
            true => b'1',
        };
        xml.push(byte)
    }
}
