use uuid::Uuid;

use crate::api::HEX;

pub fn content_type(boundary: Uuid) -> [u8; 161] {
    let mut buf = *b"multipart/related; type=\"application/xop+xml\"; boundary=\"uuid:00000000-0000-0000-0000-000000000000\"; start=\"<root.message@cxf.apache.org>\"; start-info=\"text/xml\"";

    const UUID_OFFSET: usize =
        b"multipart/related; type=\"application/xop+xml\"; boundary=\"uuid:".len();

    let mut j = UUID_OFFSET;

    for (i, &b) in boundary.as_bytes().iter().enumerate() {
        buf[j] = HEX[(b >> 4) as usize];
        buf[j + 1] = HEX[(b & 0x0F) as usize];
        j += 2;

        if matches!(i, 3 | 5 | 7 | 9) {
            buf[j] = b'-';
            j += 1;
        }
    }

    buf
}
