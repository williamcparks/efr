import { v4 as uuid } from "uuid";
import { sha1, util, pki } from "node-forge";

import type { MultiPartRequestBuilder } from "./MultiPartRequest";

const FIVE_MIN = 300000;

export const security = (
  builder: MultiPartRequestBuilder,
  privateKey: pki.rsa.PrivateKey,
) => {
  const created = new Date();
  const expires = new Date(created.getTime() + FIVE_MIN);

  const x509ReferenceUUID = uuid();
  const tsUUID = uuid();
  const sigUUID = uuid();
  const kiUUID = uuid();
  const strUUID = uuid();

  const digest = createDigest(created, expires, tsUUID);
  const signature = createSignature(tsUUID, digest, privateKey);

  const xml = `<wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" soap:mustUnderstand="1"><wsse:BinarySecurityToken xmlns:xop="http://www.w3.org/2004/08/xop/include" EncodingType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary" ValueType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3" wsu:Id="X509-${x509ReferenceUUID}"><xop:Include href="cid:${builder.cert_content_uuid}"/></wsse:BinarySecurityToken><wsu:Timestamp wsu:Id="TS-${tsUUID}"><wsu:Created>${created.toISOString()}</wsu:Created><wsu:Expires>${expires.toISOString()}</wsu:Expires></wsu:Timestamp><ds:Signature xmlns:ds="http://www.w3.org/2000/09/xmldsig#" Id="SIG-${sigUUID}"><ds:SignedInfo><ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="soap"/></ds:CanonicalizationMethod><ds:SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"/><ds:Reference URI="#TS-${tsUUID}"><ds:Transforms><ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="wsse soap"/></ds:Transform></ds:Transforms><ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/><ds:DigestValue>${digest}</ds:DigestValue></ds:Reference></ds:SignedInfo><ds:SignatureValue>${signature}</ds:SignatureValue><ds:KeyInfo Id="KI-${kiUUID}"><wsse:SecurityTokenReference xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" wsu:Id="STR-${strUUID}"><wsse:Reference URI="#X509-${x509ReferenceUUID}" ValueType="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-x509-token-profile-1.0#X509v3"/></wsse:SecurityTokenReference></ds:KeyInfo></ds:Signature></wsse:Security>`;

  builder.body += xml;
};

const createDigest = (created: Date, expires: Date, tsUUID: string): string => {
  const createdStr = created.toISOString();
  const expiresStr = expires.toISOString();

  const xml = `<wsu:Timestamp xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd" xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd" wsu:Id="TS-${tsUUID}"><wsu:Created>${createdStr}</wsu:Created><wsu:Expires>${expiresStr}</wsu:Expires></wsu:Timestamp>`;

  const md = sha1.create();
  md.update(xml, "utf8");
  return util.encode64(md.digest().bytes());
};

const createSignature = (
  tsUUID: string,
  digest: string,
  private_key: pki.rsa.PrivateKey,
): string => {
  const xml = `<ds:SignedInfo xmlns:ds="http://www.w3.org/2000/09/xmldsig#" xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"><ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="soap"></ec:InclusiveNamespaces></ds:CanonicalizationMethod><ds:SignatureMethod Algorithm="http://www.w3.org/2000/09/xmldsig#rsa-sha1"></ds:SignatureMethod><ds:Reference URI="#TS-${tsUUID}"><ds:Transforms><ds:Transform Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"><ec:InclusiveNamespaces xmlns:ec="http://www.w3.org/2001/10/xml-exc-c14n#" PrefixList="wsse soap"></ec:InclusiveNamespaces></ds:Transform></ds:Transforms><ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"></ds:DigestMethod><ds:DigestValue>${digest}</ds:DigestValue></ds:Reference></ds:SignedInfo>`;

  const md = sha1.create();
  md.update(xml, "utf8");

  const signatureBytes = private_key.sign(md);

  const signatureB64 = util.encode64(signatureBytes);

  return signatureB64;
};
