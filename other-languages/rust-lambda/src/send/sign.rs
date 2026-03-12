use rsa::{pkcs8::DecodePrivateKey, RsaPrivateKey};

use crate::send::SendError;

const PKEY_PEM: &str = include_str!("../../../../env/pkey.pem");
const CERT_DER: &[u8] = include_bytes!("../../../../env/cert.der");

pub fn sign() -> Result<(RsaPrivateKey, &'static [u8]), SendError> {
    let rsa_private_key = RsaPrivateKey::from_pkcs8_pem(PKEY_PEM)
        .map_err(|err| SendError::Rsa(err.to_string().into_boxed_str()))?;
    Ok((rsa_private_key, CERT_DER))
}
