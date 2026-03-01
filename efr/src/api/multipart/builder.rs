use uuid::Uuid;

use crate::api::Xml;

use super::MultiPartRequest;

pub(crate) struct MultiPartRequestBuilder {
    bytes: Vec<u8>,
    pub(crate) cert_content_uuid: Uuid,
    boundary: Uuid,
}

impl MultiPartRequestBuilder {
    pub(crate) fn new(size: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(size),
            cert_content_uuid: Uuid::new_v4(),
            boundary: Uuid::new_v4(),
        }
    }

    pub(crate) fn xml(&mut self) -> &mut Vec<u8> {
        let xml = &mut self.bytes;

        xml.extend_from_slice(b"\r\n--uuid:");
        self.boundary.xml(xml);
        xml.extend_from_slice(b"\r\nContent-Type: application/xop+xml; charset=UTF-8; type=\"text/xml\"\r\nContent-Transfer-Encoding: binary\r\nContent-ID: <root.message@cxf.apache.org>\r\n\r\n");

        xml
    }

    pub(crate) fn cert_der(mut self, cert_der: &[u8]) -> MultiPartRequest {
        self.bytes.extend_from_slice(b"\r\n--uuid:");
        self.boundary.xml(&mut self.bytes);
        self.bytes.extend_from_slice(b"\r\nContent-Type: application/ciphervalue\r\nContent-Transfer-Encoding: binary\r\nContent-ID: <");
        self.cert_content_uuid.xml(&mut self.bytes);
        self.bytes.extend_from_slice(b">\r\n\r\n");
        self.bytes.extend_from_slice(cert_der);
        self.bytes.extend_from_slice(b"\r\n--uuid:");
        self.boundary.xml(&mut self.bytes);
        self.bytes.extend_from_slice(b"--");

        MultiPartRequest::new(self.boundary, self.bytes)
    }

    pub(crate) const MULTI_PART_REQUEST_OVERHEAD: usize = 416;
}
