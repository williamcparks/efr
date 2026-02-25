use uuid::Uuid;

use crate::api::Xml;

pub(crate) const HEX: &[u8; 16] = b"0123456789abcdef";

impl Xml for Uuid {
    fn xml(&self, xml: &mut Vec<u8>) {
        for (i, &b) in self.as_bytes().iter().enumerate() {
            xml.push(HEX[(b >> 4) as usize]);
            xml.push(HEX[(b & 0x0F) as usize]);

            // Insert dashes at proper positions
            if matches!(i, 3 | 5 | 7 | 9) {
                xml.push(b'-');
            }
        }
    }

    fn len(&self) -> usize {
        36
    }
}
