use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::api::Xml;

impl Xml for DateTime<Utc> {
    fn xml(&self, xml: &mut Vec<u8>) {
        let year = self.year();
        let month = self.month();
        let day = self.day();
        let hour = self.hour();
        let min = self.minute();
        let sec = self.second();
        let millis = self.timestamp_subsec_millis();

        // Year (4 digits)
        xml.push(b'0' + ((year / 1000) as u8));
        xml.push(b'0' + (((year / 100) % 10) as u8));
        xml.push(b'0' + (((year / 10) % 10) as u8));
        xml.push(b'0' + ((year % 10) as u8));

        xml.push(b'-');

        // Month (2 digits)
        xml.push(b'0' + ((month / 10) as u8));
        xml.push(b'0' + ((month % 10) as u8));

        xml.push(b'-');

        // Day (2 digits)
        xml.push(b'0' + ((day / 10) as u8));
        xml.push(b'0' + ((day % 10) as u8));

        xml.push(b'T');

        // Hour (2 digits)
        xml.push(b'0' + ((hour / 10) as u8));
        xml.push(b'0' + ((hour % 10) as u8));

        xml.push(b':');

        // Minute (2 digits)
        xml.push(b'0' + ((min / 10) as u8));
        xml.push(b'0' + ((min % 10) as u8));

        xml.push(b':');

        // Second (2 digits)
        xml.push(b'0' + ((sec / 10) as u8));
        xml.push(b'0' + ((sec % 10) as u8));

        xml.push(b'.');

        // Milliseconds (3 digits)
        xml.push(b'0' + ((millis / 100) as u8));
        xml.push(b'0' + (((millis / 10) % 10) as u8));
        xml.push(b'0' + ((millis % 10) as u8));

        xml.push(b'Z');
    }

    fn len(&self) -> usize {
        24
    }
}
