import httpx

from config import Config;
from admin_service.authenticate_user_request import AuthenticateUserRequest, SOAP_ACTION
from api.metadata import TEXAS_STAGE_USER_SERVICE_URL

def main():
    config = Config.try_new()
    authenticate_user_request = AuthenticateUserRequest(
        email=config.admin.email,
        password=config.admin.password
    )
    request = authenticate_user_request.request(config)

    print("---outbound---")
    print("Content-Type:", request.content_type())
    print(request.body.decode("utf-8", errors="replace"))
    print("---/outbound---")

    with httpx.Client() as client:
        res = client.post(
            TEXAS_STAGE_USER_SERVICE_URL,
            headers={
                "Content-Type": request.content_type(),
                "SOAPAction": SOAP_ACTION,
            },
            content=request.body
        )

        print("---inbound---")
        print(res, res.headers)
        print(res.text)
        print("---/inbound---")


if __name__ == "__main__":
    main()