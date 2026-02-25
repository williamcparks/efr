pub(super) struct ApiError {
    pub(super) code: i64,
    pub(super) message: Box<str>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Init,
    InError,
    InCode,
    InMessage,
}

impl ApiError {
    pub(super) fn new_opt(xml: &str) -> Option<Self> {
        let mut reader = quick_xml::Reader::from_str(xml);

        let mut state = State::Init;
        let mut code: Option<i64> = None;

        loop {
            match reader.read_event().ok()? {
                quick_xml::events::Event::Start(e) => {
                    let tag = e.local_name();
                    let tag = tag.as_ref();
                    state = match tag {
                        b"Error" => State::InError,
                        b"ErrorCode" if state == State::InError => State::InCode,
                        b"ErrorText" if state == State::InError => State::InMessage,
                        _ => continue,
                    };
                }
                quick_xml::events::Event::Text(e) => {
                    let text = e.xml10_content().ok()?;

                    match state {
                        State::InCode => {
                            code = text.parse::<i64>().ok();
                            if code == Some(0) {
                                return None;
                            }
                        }
                        State::InMessage => {
                            return Some(Self {
                                code: code?,
                                message: text.into(),
                            });
                        }
                        _ => {}
                    }
                }
                quick_xml::events::Event::End(e) => {
                    let tag = e.name();
                    let tag = tag.as_ref();

                    state = match tag {
                        b"ErrorCode" => State::InError,
                        _ => State::Init,
                    };
                }
                quick_xml::events::Event::Eof => break,
                _ => {}
            }
        }

        None
    }
}
