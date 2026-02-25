from dataclasses import dataclass

from config import Config;
from api.multipart_request_builder import MultiPartRequestBuilder
from api.multipart_request import MultiPartRequest
from api.envelope import ENVELOPE_START, END_HEADER_START_BODY, END_BODY
from api.security import security

@dataclass
class AuthenticateUserRequest:
    email: str
    password: str

    def request(self, config: Config) -> MultiPartRequest:
        builder = MultiPartRequestBuilder.new()

        builder.body += ENVELOPE_START
        security(builder, config)
        builder.body += END_HEADER_START_BODY
        builder.body += self.xml()
        builder.body += END_BODY

        return builder.build(config.credentials.cert_der)

    def xml(self) -> str:
        return f"""<tyler:AuthenticateUser xmlns:tyler="urn:tyler:efm:services" xmlns:auth="urn:tyler:efm:services:schema:AuthenticateRequest"><tyler:AuthenticateRequest><auth:Email>{self.email}</auth:Email><auth:Password>{self.password}</auth:Password></tyler:AuthenticateRequest></tyler:AuthenticateUser>"""


SOAP_ACTION = "urn:tyler:efm:services/IEfmUserService/AuthenticateUser"