use std::borrow::Cow;

pub(super) struct FaultError {
    pub(super) code: Box<str>,
    pub(super) message: Box<str>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum State {
    Init,
    InFault,
    InCode,
    InMessage,
}

impl FaultError {
    pub(super) fn new_opt(xml: &str) -> Option<Self> {
        let mut reader = quick_xml::Reader::from_str(xml);

        let mut state = State::Init;
        let mut code: Option<Cow<'_, str>> = None;

        loop {
            match reader.read_event().ok()? {
                quick_xml::events::Event::Start(e) => {
                    let tag = e.local_name();
                    let tag = tag.as_ref();

                    state = match tag {
                        b"Fault" => State::InFault,
                        b"faultcode" if state == State::InFault => State::InCode,
                        b"faultstring" if state == State::InFault => State::InMessage,
                        _ => continue,
                    };
                }
                quick_xml::events::Event::Text(e) => {
                    let text = e.xml10_content().ok()?;

                    match state {
                        State::InCode => {
                            code = Some(text);
                        }
                        State::InMessage => {
                            return Some(Self {
                                code: code?.into(),
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
                        b"faultcode" => State::InFault,
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
