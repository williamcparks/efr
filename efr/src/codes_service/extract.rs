use quick_xml::{Reader, events::Event};

use crate::codes_service::EfrCodesError;

struct State {
    start: u64,
    end: u64,
}

pub(super) fn extract<'a>(
    tag: &'static str,
    xml: &'a str,
    reader: &mut Reader<&'_ [u8]>,
) -> Result<Option<&'a str>, EfrCodesError> {
    let tag_bytes = tag.as_bytes();
    let mut state = State { start: 0, end: 0 };

    loop {
        let previous_position = reader.buffer_position();

        match reader.read_event()? {
            Event::Start(start) => {
                let local_name = start.local_name();
                let local_tag = local_name.as_ref();
                if local_tag == tag_bytes {
                    state.start = previous_position;
                }
            }
            Event::End(end) => {
                let local_name = end.local_name();
                let local_tag = local_name.as_ref();
                if local_tag == tag_bytes {
                    state.end = reader.buffer_position();
                    break;
                }
            }
            Event::Empty(empty) => {
                let local_name = empty.local_name();
                let local_tag = local_name.as_ref();
                if local_tag == tag_bytes {
                    state = State {
                        start: previous_position,
                        end: reader.buffer_position(),
                    };
                    break;
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    if state.end == 0 {
        return Ok(None);
    }

    let start = state.start as usize;
    let end = state.end as usize;

    Ok(Some(&xml[start..end]))
}
