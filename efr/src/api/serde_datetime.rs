use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserializer, de::Visitor};

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

struct Vis;

impl<'de> Visitor<'de> for Vis {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an RFC 3339 formatted date and time string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let naive = NaiveDateTime::parse_from_str(v, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(naive.and_utc())
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(Vis)
}
