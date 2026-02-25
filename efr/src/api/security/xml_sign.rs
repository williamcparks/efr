use base64::Engine;
use efr_macros::partial_xml;
use rsa::{
    pkcs1v15::SigningKey,
    signature::{SignatureEncoding, Signer},
};
use sha1::Sha1;
use uuid::Uuid;

use crate::api::Xml;

use super::copy_from_same_vec::copy_from_same_vec;

pub(super) fn xml_sign(
    ts_uuid: Uuid,
    digest_end: usize,
    signing_key: &SigningKey<Sha1>,
    xml: &mut Vec<u8>,
) {
    let len = xml.len();

    xml.extend_from_slice(PART_0);
    ts_uuid.xml(xml);
    xml.extend_from_slice(PART_1);
    copy_from_same_vec(xml, digest_end - 28, 28);
    xml.extend_from_slice(PART_2);
    let signature = signing_key.sign(&xml[len..]).to_bytes();

    xml.truncate(len);

    let sign_len = signature.len().div_ceil(3) * 4;
    xml.resize(len + sign_len, 0);
    let _ = base64::engine::general_purpose::STANDARD.encode_slice(signature, &mut xml[len..]);
}

const PART_0: &[u8] = partial_xml! {
     <ds:SignedInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
        <ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#">
            <ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="soap"></ec:InclusiveNamespaces>
        </ds:CanonicalizationMethod>
        <ds:SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"></ds:SignatureMethod>
        <ds:Reference URI=$"#TS-"
};

const PART_1: &[u8] = partial_xml! {
    $"">
    <ds:Transforms>
        <ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#">
            <ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="wsse soap"></ec:InclusiveNamespaces>
        </ds:Transform>
    </ds:Transforms>
    <ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"></ds:DigestMethod>
    <ds:DigestValue>
};

const PART_2: &[u8] = partial_xml! {
    </ds:DigestValue>
    </ds:Reference>
    </ds:SignedInfo>
};
