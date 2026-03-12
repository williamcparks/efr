use quick_xml::name::ResolveResult;

fn qname<'a, 'b>(local_name: &'a [u8], uri: &'b ResolveResult) -> (&'a str, &'b str) {
    let local_name = str::from_utf8(local_name).unwrap_or("unknown");

    let uri = match uri {
        ResolveResult::Bound(ns) => ns.into_inner(),
        ResolveResult::Unbound | ResolveResult::Unknown(_) => return (local_name, ""),
    };

    let prefix = match uri {
        b"http://release.niem.gov/niem/niem-core/4.0/" => "nc",
        b"http://release.niem.gov/niem/domains/jxdm/6.1/" => "j",
        b"https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/ecf" => "ecf",
        b"https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/wrappers" => "wrappers",
        b"https://docs.oasis-open.org/legalxml-courtfiling/ns/v5.0/caselistresponse" => {
            "caselistresponse"
        }
        b"http://release.niem.gov/niem/domains/cbrn/4.1/" => "cbrn",
        _ => {
            let value = str::from_utf8(uri).unwrap_or("unknown");

            #[cfg(debug_assertions)]
            panic!("unknown uri: {}", value);

            #[allow(unused)]
            value
        }
    };

    (local_name, prefix)
}

pub fn json_key(local_name: &[u8], uri: &ResolveResult) -> String {
    let (local_name, uri) = qname(local_name, uri);

    let mut buf = String::with_capacity(local_name.len() + uri.len() + 1);

    buf.push_str(uri);
    buf.push(':');
    buf.push_str(local_name);

    buf
}

pub fn json_key_attr(local_name: &[u8], uri: &ResolveResult) -> String {
    let (local_name, uri) = qname(local_name, uri);

    let mut buf = String::with_capacity(local_name.len() + uri.len() + 2);

    buf.push('@');
    buf.push_str(uri);
    buf.push(':');
    buf.push_str(local_name);

    buf
}
