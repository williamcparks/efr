use uuid::Uuid;

pub struct MultiPartRequest {
    content_type: [u8; 161],
    bytes: Vec<u8>,
}

impl MultiPartRequest {
    pub(super) fn new(boundary: Uuid, bytes: Vec<u8>) -> Self {
        Self {
            content_type: super::content_type::content_type(boundary),
            bytes,
        }
    }

    pub fn content_type(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.content_type.as_slice()) }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn into_content_type(self) -> [u8; 161] {
        self.content_type
    }

    pub fn into_parts(self) -> (Vec<u8>, [u8; 161]) {
        (self.bytes, self.content_type)
    }
}
