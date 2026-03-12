use lambda_http::Request;

use crate::send::SendError;

pub struct AuthToken {
    header: Box<str>,
    split_idx: usize,
}

impl AuthToken {
    pub fn new_opt(request: &Request) -> Option<Self> {
        let headers = request.headers();
        let header = headers.get("authtoken")?.to_str().ok()?;

        let split_idx = header.find(':')?;

        Some(Self {
            header: header.into(),
            split_idx,
        })
    }

    pub fn enforce(opt: Option<Self>) -> Result<Self, SendError> {
        opt.ok_or(SendError::AuthToken)
    }

    pub fn pair(&self) -> (&str, &str) {
        self.header.split_at(self.split_idx)
    }
}
