from dataclasses import dataclass
from uuid import UUID, uuid4

from api.multipart_request import MultiPartRequest

@dataclass
class MultiPartRequestBuilder:
    body: str
    cert_content_uuid: UUID
    boundary: UUID

    @staticmethod
    def new():
        boundary = uuid4()
        body = f"\r\n--uuid:{boundary}\r\nContent-Type: application/xop+xml; charset=UTF-8; type=\"text/xml\"\r\nContent-Transfer-Encoding: binary\r\nContent-ID: <root.message@cxf.apache.org>\r\n\r\n"

        res = MultiPartRequestBuilder(
            body=body,
            cert_content_uuid=uuid4(),
            boundary=boundary
        )

        return res

    def build(self, cert_der: bytes) -> MultiPartRequest:
        self.body += f"\r\n--uuid:{self.boundary}\r\nContent-Type: application/ciphervalue\r\nContent-Transfer-Encoding: binary\r\nContent-ID: <{self.cert_content_uuid}>\r\n\r\n"
        body = bytes(self.body, "utf-8")
        body += cert_der
        body += bytes(f"\r\n--uuid:{self.boundary}--", "utf-8")

        return MultiPartRequest(
            body=body,
            boundary=self.boundary
        )