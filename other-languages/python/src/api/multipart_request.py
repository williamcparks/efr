from dataclasses import dataclass
from uuid import UUID

@dataclass
class MultiPartRequest:
    body: bytes
    boundary: UUID

    def content_type(self):
        return f"multipart/related; type=\"application/xop+xml\"; boundary=\"uuid:{self.boundary}\"; start=\"<root.message@cxf.apache.org>\"; start-info=\"text/xml\""
