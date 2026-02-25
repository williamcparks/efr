import hashlib
import base64

from uuid import UUID, uuid4
from datetime import datetime, timedelta, timezone

from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import padding

from api.multipart_request_builder import MultiPartRequestBuilder
from config import Config

def security(builder: MultiPartRequestBuilder, config: Config):
    created = datetime.now(timezone.utc)
    expires = created + timedelta(minutes=5)
    created_iso = rfc3339(created)
    expires_iso = rfc3339(expires)

    x509_uuid = uuid4()
    ts_uuid = uuid4()
    sig_uuid = uuid4()
    ki_uuid = uuid4()
    str_uuid = uuid4()

    digest = create_digest(created_iso, expires_iso, ts_uuid)
    signature = create_signature(digest, ts_uuid, config.credentials.pkey_pem)

    xml = f"""<wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" soap:mustUnderstand="1"><wsse:BinarySecurityToken xmlns:xop="http://www.w3.org/2004/08/xop/include" EncodingType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary" ValueType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3" wsu:Id="X509-{x509_uuid}"><xop:Include href="cid:{builder.cert_content_uuid}"/></wsse:BinarySecurityToken><wsu:Timestamp wsu:Id="TS-{ts_uuid}"><wsu:Created>{created_iso}</wsu:Created><wsu:Expires>{expires_iso}</wsu:Expires></wsu:Timestamp><ds:Signature xmlns:ds="http://www.w3.org/2000/09/xmldsig#" Id="SIG-{sig_uuid}"><ds:SignedInfo><ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="soap"/></ds:CanonicalizationMethod><ds:SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><ds:Reference URI="#TS-{ts_uuid}"><ds:Transforms><ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="wsse soap"/></ds:Transform></ds:Transforms><ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><ds:DigestValue>{digest}</ds:DigestValue></ds:Reference></ds:SignedInfo><ds:SignatureValue>{signature}</ds:SignatureValue><ds:KeyInfo Id="KI-{ki_uuid}"><wsse:SecurityTokenReference xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" wsu:Id="STR-{str_uuid}"><wsse:Reference URI="#X509-{x509_uuid}" ValueType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3"/></wsse:SecurityTokenReference></ds:KeyInfo></ds:Signature></wsse:Security>"""
    builder.body += xml

def create_digest(created_iso: str, expires_iso: str, ts_uuid: UUID) -> str:
    xml = f"""<wsu:Timestamp xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" wsu:Id="TS-{ts_uuid}"><wsu:Created>{created_iso}</wsu:Created><wsu:Expires>{expires_iso}</wsu:Expires></wsu:Timestamp>"""
    digest_bytes = hashlib.sha1(xml.encode("utf-8")).digest()
    return base64.b64encode(digest_bytes).decode("utf-8")

def create_signature(digest: str, ts_uuid: UUID, private_key: RSAPrivateKey) -> str:
    xml = f"""<ds:SignedInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"><ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="soap"></ec:InclusiveNamespaces></ds:CanonicalizationMethod><ds:SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"></ds:SignatureMethod><ds:Reference URI="#TS-{ts_uuid}"><ds:Transforms><ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="wsse soap"></ec:InclusiveNamespaces></ds:Transform></ds:Transforms><ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"></ds:DigestMethod><ds:DigestValue>{digest}</ds:DigestValue></ds:Reference></ds:SignedInfo>"""
    s = private_key.sign(xml.encode("utf-8"), padding.PKCS1v15(), hashes.SHA1())
    return base64.b64encode(s).decode("utf-8")

def rfc3339(dt: datetime) -> str:
    return dt.strftime("%Y-%m-%dT%H:%M:%S.%fZ")