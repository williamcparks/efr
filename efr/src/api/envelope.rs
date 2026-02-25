pub(crate) struct Envelope;

impl Envelope {
    pub(crate) const START_HEADER: &'static [u8] =
        br##"<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"><soap:Header>"##;

    pub(crate) const END_HEADER_START_BODY: &'static [u8] = br##"</soap:Header><soap:Body>"##;

    pub(crate) const END_BODY: &'static [u8] = br##"</soap:Body></soap:Envelope>"##;

    pub(crate) const ENVELOPE_OVERHEAD: usize =
        Self::START_HEADER.len() + Self::END_HEADER_START_BODY.len() + Self::END_BODY.len();
}
