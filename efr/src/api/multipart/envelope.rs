use rsa::{pkcs1v15::SigningKey, traits::PublicKeyParts};
use sha1::Sha1;

use crate::api::{
    Envelope, MultiPartRequest, MultiPartRequestBuilder, SecurityHeader, TylerHeader, Xml,
};

impl MultiPartRequest {
    pub fn envelope_insecure(body: &str, signing_key: &SigningKey<Sha1>, cert_der: &[u8]) -> Self {
        let len = MultiPartRequestBuilder::MULTI_PART_REQUEST_OVERHEAD
            + Envelope::ENVELOPE_OVERHEAD
            + SecurityHeader::SECURITY_HEADER_OVERHEAD
            + signing_key.as_ref().size().div_ceil(3) * 4
            + cert_der.len()
            + body.len();

        let mut builder = MultiPartRequestBuilder::new(len);
        let sec = SecurityHeader::new(signing_key, builder.cert_content_uuid);

        let xml = builder.xml();
        xml.extend_from_slice(Envelope::START_HEADER);
        sec.xml(xml);
        xml.extend_from_slice(Envelope::END_HEADER_START_BODY);
        xml.extend_from_slice(body.as_bytes());
        xml.extend_from_slice(Envelope::END_BODY);

        builder.cert_der(cert_der)
    }

    pub fn envelope_secure(
        body: &str,
        email: &str,
        password_hash: &str,
        signing_key: &SigningKey<Sha1>,
        cert_der: &[u8],
    ) -> Self {
        let tyler = TylerHeader {
            email,
            password_hash,
        };

        let len = MultiPartRequestBuilder::MULTI_PART_REQUEST_OVERHEAD
            + Envelope::ENVELOPE_OVERHEAD
            + SecurityHeader::SECURITY_HEADER_OVERHEAD
            + tyler.len()
            + signing_key.as_ref().size().div_ceil(3) * 4
            + cert_der.len()
            + body.len();

        let mut builder = MultiPartRequestBuilder::new(len);
        let sec = SecurityHeader::new(signing_key, builder.cert_content_uuid);

        let xml = builder.xml();
        xml.extend_from_slice(Envelope::START_HEADER);
        tyler.xml(xml);
        sec.xml(xml);
        xml.extend_from_slice(Envelope::END_HEADER_START_BODY);
        xml.extend_from_slice(body.as_bytes());
        xml.extend_from_slice(Envelope::END_BODY);

        builder.cert_der(cert_der)
    }
}
