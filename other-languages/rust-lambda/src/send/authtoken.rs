use lambda_http::Request;

use crate::send::SendError;

pub struct AuthToken {
    header: Box<str>,
    colon_idx: usize,
}

impl AuthToken {
    pub fn try_new_opt(request: &Request) -> Result<Option<Self>, SendError> {
        let headers = request.headers();
        let header = match headers.get("authtoken") {
            Some(some) => some,
            None => return Ok(None),
        };
        let header = match header.to_str() {
            Ok(ok) => ok,
            Err(_) => return Err(SendError::AuthToken),
        };

        let colon_idx = match header.find(':') {
            Some(some) => some,
            None => return Err(SendError::AuthToken),
        };

        Ok(Some(Self {
            header: header.into(),
            colon_idx,
        }))
    }

    pub fn pair(&self) -> (&str, &str) {
        let email = unsafe { self.header.get_unchecked(..self.colon_idx) };
        let password = unsafe { self.header.get_unchecked(self.colon_idx + 1..) };
        (email, password)
    }
}
