import { v4 as uuid } from "uuid";

export interface MultiPartRequestBuilder {
  boundary: string;
  cert_content_uuid: string;
  body: string;
}

export interface MultiPartRequest {
  boundary: string;
  body: Uint8Array;
}

export const create = (): MultiPartRequestBuilder => {
  const boundary = uuid();
  return {
    boundary,
    cert_content_uuid: uuid(),
    body: `\r\n--uuid:${boundary}\r\nContent-Type: application/xop+xml; charset=UTF-8; type=\"text/xml\"\r\nContent-Transfer-Encoding: binary\r\nContent-ID: <root.message@cxf.apache.org>\r\n\r\n`,
  };
};

export const contentType = (multipart: MultiPartRequest) =>
  `multipart/related; type=\"application/xop+xml\"; boundary=\"uuid:${multipart.boundary}\"; start=\"<root.message@cxf.apache.org>\"; start-info=\"text/xml\"`;

export const certDer = (
  multipart: MultiPartRequestBuilder,
  der: Uint8Array,
): MultiPartRequest => {
  const encoder = new TextEncoder();
  const start = encoder.encode(
    multipart.body +
      `\r\n--uuid:${multipart.boundary}\r\nContent-Type: application/ciphervalue\r\nContent-Transfer-Encoding: binary\r\nContent-ID: <${multipart.cert_content_uuid}>\r\n\r\n`,
  );
  const end = encoder.encode(`\r\n--uuid:${multipart.boundary}--\r\n`);
  const result = new Uint8Array(start.length + der.length + end.length);
  result.set(start, 0);
  result.set(der, start.length);
  result.set(end, start.length + der.length);

  return {
    boundary: multipart.boundary,
    body: result,
  };
};
