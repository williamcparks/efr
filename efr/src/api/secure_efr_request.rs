use rsa::{pkcs1v15::SigningKey, traits::PublicKeyParts};
use sha1::Sha1;

use crate::api::{
    EfrRequest, Envelope, MultiPartRequest, MultiPartRequestBuilder, SecurityHeader, TylerHeader,
    Xml,
};

pub(crate) trait SecureEfrRequest: Xml {
    const SOAP_ACTION: &'static str;

    fn email(&self) -> &str;

    fn password_hash(&self) -> &str;
}

impl<T: SecureEfrRequest> EfrRequest for T {
    const SOAP_ACTION: &'static str = T::SOAP_ACTION;

    fn efr_request(&self, signing_key: &SigningKey<Sha1>, cert_der: &[u8]) -> MultiPartRequest {
        let tyler = TylerHeader {
            email: self.email(),
            password_hash: self.password_hash(),
        };

        let len = MultiPartRequestBuilder::MUTLI_PART_REQUEST_OVERHEAD
            + Envelope::ENVELOPE_OVERHEAD
            + SecurityHeader::SECURITY_HEADER_OVERHEAD
            + tyler.len()
            + signing_key.as_ref().size().div_ceil(3) * 4
            + cert_der.len()
            + self.len();

        let mut builder = MultiPartRequestBuilder::new(len);
        let sec = SecurityHeader::new(signing_key, builder.cert_content_uuid);

        let xml = builder.xml();
        xml.extend_from_slice(Envelope::START_HEADER);
        tyler.xml(xml);
        sec.xml(xml);
        xml.extend_from_slice(Envelope::END_HEADER_START_BODY);
        self.xml(xml);
        xml.extend_from_slice(Envelope::END_BODY);

        builder.cert_der(cert_der)
    }
}
