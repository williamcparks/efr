use crate::api::{EfrError, MultiPartResponse};

pub fn json(multipart_response: &str) -> Result<Vec<u8>, EfrError> {
    let xml = MultiPartResponse::try_new(multipart_response)?;

    let mut out = Vec::with_capacity(4096);
    quick_xml_to_json::xml_to_json(xml.xml.as_bytes(), &mut out)?;

    Ok(out)
}
