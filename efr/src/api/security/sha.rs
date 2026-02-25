use base64::Engine;
use chrono::{DateTime, Utc};
use efr_macros::partial_xml;
use sha1::{Digest, Sha1};
use uuid::Uuid;

use crate::api::Xml;

pub(super) struct Sha {
    created: DateTime<Utc>,
    expires: DateTime<Utc>,
    ts_uuid: Uuid,
}

impl Sha {
    pub(super) fn new(created: DateTime<Utc>, expires: DateTime<Utc>, ts_uuid: Uuid) -> Self {
        Self {
            created,
            expires,
            ts_uuid,
        }
    }
}

impl Xml for Sha {
    fn xml(&self, xml: &mut Vec<u8>) {
        let len = xml.len();

        xml.extend_from_slice(PART_0);
        self.ts_uuid.xml(xml);
        xml.extend_from_slice(PART_1);
        self.created.xml(xml);
        xml.extend_from_slice(PART_2);
        self.expires.xml(xml);
        xml.extend_from_slice(PART_3);

        let mut hasher = Sha1::new();
        hasher.update(&xml[len..]);
        let digest = hasher.finalize();
        xml.truncate(len);

        xml.resize(len + 28, 0);
        let _ = base64::engine::general_purpose::STANDARD.encode_slice(digest, &mut xml[len..]);
    }

    fn len(&self) -> usize {
        28
    }
}

const PART_0: &[u8] = partial_xml! {
    <wsu:Timestamp
        xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/"
        xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd"
        xmlns:wsu="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd"
        wsu:Id=$"TS-"
};

const PART_1: &[u8] = partial_xml! {
    $""><wsu:Created>
};

const PART_2: &[u8] = partial_xml! {
    </wsu:Created><wsu:Expires>
};

const PART_3: &[u8] = partial_xml! {
    </wsu:Expires></wsu:Timestamp>
};
