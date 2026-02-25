use chrono::{Duration, Utc};
use efr_macros::partial_xml;
use rsa::{pkcs1v15::SigningKey, traits::PublicKeyParts};
use sha1::Sha1;
use uuid::Uuid;

use crate::api::Xml;

use super::{sha::Sha, xml_sign::xml_sign};

pub(crate) struct SecurityHeader<'a> {
    signing_key: &'a SigningKey<Sha1>,
    cert_content_uuid: Uuid,
}

impl<'a> SecurityHeader<'a> {
    pub(crate) const SECURITY_HEADER_OVERHEAD: usize = 2269;

    pub(crate) fn new(signing_key: &'a SigningKey<Sha1>, cert_content_uuid: Uuid) -> Self {
        Self {
            signing_key,
            cert_content_uuid,
        }
    }
}

impl<'a> Xml for SecurityHeader<'a> {
    fn xml(&self, xml: &mut Vec<u8>) {
        let created = Utc::now();
        let expires = created + Duration::minutes(5);

        let x509_reference_uuid = Uuid::new_v4();
        let ts_uuid = Uuid::new_v4();
        let sig_uuid = Uuid::new_v4();
        let ki_uuid = Uuid::new_v4();
        let str_uuid = Uuid::new_v4();

        xml.extend_from_slice(PART_0);
        x509_reference_uuid.xml(xml);
        xml.extend_from_slice(PART_1);
        self.cert_content_uuid.xml(xml);
        xml.extend_from_slice(PART_2);
        ts_uuid.xml(xml);
        xml.extend_from_slice(PART_3);
        created.xml(xml);
        xml.extend_from_slice(PART_4);
        expires.xml(xml);
        xml.extend_from_slice(PART_5);
        sig_uuid.xml(xml);
        xml.extend_from_slice(PART_6);
        ts_uuid.xml(xml);
        xml.extend_from_slice(PART_7);

        let sha = Sha::new(created, expires, ts_uuid);
        sha.xml(xml);

        xml.extend_from_slice(PART_8);

        let digest_end = xml.len() - PART_8.len();
        xml_sign(ts_uuid, digest_end, self.signing_key, xml);

        xml.extend_from_slice(PART_9);
        ki_uuid.xml(xml);
        xml.extend_from_slice(PART_10);
        str_uuid.xml(xml);
        xml.extend_from_slice(PART_11);
        x509_reference_uuid.xml(xml);
        xml.extend_from_slice(PART_12);
    }

    fn len(&self) -> usize {
        Self::SECURITY_HEADER_OVERHEAD + self.signing_key.as_ref().size().div_ceil(3) * 4
    }
}

const PART_0: &[u8] = partial_xml! {
    <wsse:Security
        xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd"
        xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd"
        soap:mustUnderstand="1">
            <wsse:BinarySecurityToken xmlns:xop="http://www.w3.org/2004/08/xop/include"
                EncodingType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary"
                ValueType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3"
                wsu:Id=$"X509-"
};

const PART_1: &[u8] = partial_xml! {
    $""><xop:Include href=$"cid:"
};

const PART_2: &[u8] = partial_xml! {
    $"" /></wsse:BinarySecurityToken>
    <wsu:Timestamp wsu:Id=$"TS-"
};

const PART_3: &[u8] = partial_xml! {
    $""><wsu:Created>
};

const PART_4: &[u8] = partial_xml! {
    </wsu:Created><wsu:Expires>
};

const PART_5: &[u8] = partial_xml! {
    </wsu:Expires></wsu:Timestamp>
    <ds:Signature xmlns:ds="http://www.w3.org/2000/09/xmldsig#"
        Id=$"SIG-"
};

const PART_6: &[u8] = partial_xml! {
    $"">
    <ds:SignedInfo>
        <ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#">
            <ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="soap" />
        </ds:CanonicalizationMethod>
        <ds:SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1" />
            <ds:Reference URI=$"#TS-"
};

const PART_7: &[u8] = partial_xml! {
    $"">
    <ds:Transforms>
        <ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#">
            <ec:InclusiveNamespaces
                xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#"
                PrefixList="wsse soap" />
            </ds:Transform>
        </ds:Transforms>
    <ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1" />
    <ds:DigestValue>
};

const PART_8: &[u8] = partial_xml! {
    </ds:DigestValue></ds:Reference></ds:SignedInfo><ds:SignatureValue>
};

const PART_9: &[u8] = partial_xml! {
    </ds:SignatureValue><ds:KeyInfo Id=$"KI-"
};

const PART_10: &[u8] = partial_xml! {
    $"">
    <wsse:SecurityTokenReference
        xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd"
        xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd"
        wsu:Id=$"STR-"
};

const PART_11: &[u8] = partial_xml! {
    $"">
    <wsse:Reference URI=$"#X509-"
};

const PART_12: &[u8] = b"\" ValueType=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3\"/></wsse:SecurityTokenReference></ds:KeyInfo></ds:Signature></wsse:Security>";
