use base64::Engine;
use chrono::{SecondsFormat, Utc};
use cms::{
    cert::CertificateChoices,
    content_info::{CmsVersion, ContentInfo},
    signed_data::{
        CertificateSet, DigestAlgorithmIdentifiers, EncapsulatedContentInfo, SignedData,
        SignerIdentifier, SignerInfo,
    },
};
use const_oid::db::rfc5911::{ID_DATA, ID_SIGNED_DATA};
use der::{Any, Encode, asn1::OctetString};
use rsa::{
    pkcs1v15::SigningKey,
    signature::{SignatureEncoding, Signer},
};
use sha2::Sha256;
use x509_cert::{Certificate, der::Decode, spki::AlgorithmIdentifierOwned};

use crate::codes_service::EfrCodesHeaderError;

pub struct CodesHeader {
    pub header: String,
}

impl CodesHeader {
    pub const TYL_EFM_API_HEADER: &str = "tyl-efm-api";

    pub fn try_new(
        cert_der: &[u8],
        signing_key: &SigningKey<Sha256>,
    ) -> Result<Self, EfrCodesHeaderError> {
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let data = timestamp.as_bytes();

        let econtent = OctetString::new(data)?.to_der()?;
        let eci = EncapsulatedContentInfo {
            econtent_type: ID_DATA,
            econtent: Some(Any::from_der(&econtent)?),
        };

        let signature = signing_key.sign(data);
        let signature_bytes = signature.to_vec();

        let cert = Certificate::from_der(cert_der)?;
        let signer_info = create_signer_info(&cert, &signature_bytes)?;

        let cert_choice = CertificateChoices::Certificate(cert.clone());
        let certificates = Some(CertificateSet::try_from(vec![cert_choice])?);

        let digest_alg_id = AlgorithmIdentifierOwned {
            oid: const_oid::db::rfc5912::ID_SHA_256,
            parameters: None,
        };

        let digest_algorithms = DigestAlgorithmIdentifiers::try_from(vec![digest_alg_id])?;

        let signed_data = SignedData {
            version: CmsVersion::V1,
            digest_algorithms,
            encap_content_info: eci,
            certificates,
            crls: None,
            signer_infos: vec![signer_info].try_into()?,
        };

        let content_info = ContentInfo {
            content_type: ID_SIGNED_DATA,
            content: Any::from_der(&signed_data.to_der()?)?,
        };

        let output_bytes = content_info.to_der()?;

        let b64 = base64::engine::general_purpose::STANDARD.encode(output_bytes.as_slice());

        Ok(Self { header: b64 })
    }

    pub fn as_str(&self) -> &str {
        self.header.as_str()
    }
}

fn create_signer_info(
    cert: &Certificate,
    signature: &[u8],
) -> Result<SignerInfo, EfrCodesHeaderError> {
    let issuer = cert.tbs_certificate.issuer.clone();
    let serial_number = cert.tbs_certificate.serial_number.clone();

    let sid = SignerIdentifier::IssuerAndSerialNumber(cms::cert::IssuerAndSerialNumber {
        issuer,
        serial_number,
    });

    let digest_alg = AlgorithmIdentifierOwned {
        oid: const_oid::db::rfc5912::ID_SHA_256,
        parameters: None,
    };

    let signature_algorithm = AlgorithmIdentifierOwned {
        oid: const_oid::db::rfc5912::RSA_ENCRYPTION,
        parameters: None,
    };

    let signer_info = SignerInfo {
        version: CmsVersion::V1,
        sid,
        digest_alg,
        signed_attrs: None,
        signature_algorithm,
        signature: OctetString::new(signature)?,
        unsigned_attrs: None,
    };

    Ok(signer_info)
}
