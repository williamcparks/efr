use crate::api::EfrError;

pub(crate) struct MultiPartResponse<'a> {
    pub xml: &'a str,
}

impl<'a> MultiPartResponse<'a> {
    pub(crate) fn try_new(src: &'a str) -> Result<Self, EfrError> {
        let src = src.trim_start();

        // Infer boundary: first line should be "--<boundary>"
        let boundary = match src.lines().next() {
            Some(some) => some,
            None => return Err(EfrError::InvalidMultiPartResponse(src.into())),
        };
        if !boundary.starts_with("--") {
            return Err(EfrError::InvalidMultiPartResponse(src.into()));
        }

        // Skip past the first boundary line (boundary + \r\n)
        let rest = match src.get(boundary.len() + 2..) {
            Some(some) => some,
            None => return Err(EfrError::InvalidMultiPartResponse(src.into())),
        }; // +2 for \r\n

        // Skip headers (or empty header section) to get to body
        let (_, body) = match rest.split_once("\r\n\r\n") {
            Some(some) => some,
            None => return Err(EfrError::InvalidMultiPartResponse(src.into())),
        };

        let (body, _) = match body.split_once(boundary) {
            Some(some) => some,
            None => return Err(EfrError::InvalidMultiPartResponse(src.into())),
        };

        // Trim the trailing \r\n before the next boundary
        Ok(Self {
            xml: body.trim_end_matches("\r\n"),
        })
    }
}
