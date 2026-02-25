import { contentType } from "./api/MultiPartRequest";
import {
  CONTENT_TYPE,
  SOAPACTION,
  TEXAS_STAGE_USER_SERVICE_URL,
} from "./api/metadata";
import { tryReadConfig } from "./config";
import {
  AUTHENTICATE_USER_SOAP_ACTION,
  authenticateUserRequest,
} from "./userService/authenticateUserRequest";

const main = async () => {
  const config = await tryReadConfig();

  const body = authenticateUserRequest(
    {
      email: config.admin.email,
      password: config.admin.password,
    },
    config.credentials.pkeyPEM,
    config.credentials.certDER,
  );

  const res = await fetch(TEXAS_STAGE_USER_SERVICE_URL, {
    method: "POST",
    headers: {
      [CONTENT_TYPE]: contentType(body),
      [SOAPACTION]: AUTHENTICATE_USER_SOAP_ACTION,
    },
    body: body.body as any,
  });
  console.log(res);
  console.log(await res.text());
};

main().catch(console.error);
