import { pki } from "node-forge";

import {
  ENVELOPE_START,
  END_BODY,
  END_HEADER_START_BODY,
} from "../api/envelope";
import { certDer, create, MultiPartRequest } from "../api/MultiPartRequest";
import { security } from "../api/security";

interface Props {
  email: string;
  password: string;
}

export const authenticateUserRequest = (
  props: Props,
  privateKey: pki.rsa.PrivateKey,
  certDER: Uint8Array,
): MultiPartRequest => {
  const builder = create();

  builder.body += ENVELOPE_START;
  security(builder, privateKey);
  builder.body += END_HEADER_START_BODY;
  builder.body += serde(props);
  builder.body += END_BODY;

  return certDer(builder, certDER);
};

const serde = ({ email, password }: Props) => {
  return `
        <tyler:AuthenticateUser xmlns:tyler="urn:tyler:efm:services"
            xmlns:auth="urn:tyler:efm:services:schema:AuthenticateRequest">
            <tyler:AuthenticateRequest>
                <auth:Email>${email}</auth:Email>
                <auth:Password>${password}</auth:Password>
            </tyler:AuthenticateRequest>
        </tyler:AuthenticateUser>
  `;
};

export const AUTHENTICATE_USER_SOAP_ACTION =
  "urn:tyler:efm:services/IEfmUserService/AuthenticateUser";
