use bitflags::bitflags;
use serde::{Deserialize, Serialize, de::Visitor};

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct NotificationPreferencesFlags: u8 {
        const ACCEPTED = 1 << 0;
        const REJECTED = 1 << 1;
        const SUBMITTED = 1 << 2;
        const SERVICEUNDELIVERABLE = 1 << 3;
        const SUBMISSIONFAILED = 1 << 4;
        const RECEIPTED = 1 << 5;
        const RETURNFORCORRECTION = 1 << 6;
        const ACCOUNTLOCKED = 1 << 7;
    }
}

impl Serialize for NotificationPreferencesFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

struct Vis;

impl<'de> Visitor<'de> for Vis {
    type Value = NotificationPreferencesFlags;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a u8 representing bitflags for `NotificationPreferencesFlags`")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(NotificationPreferencesFlags::from_bits_truncate(v))
    }
}

impl<'de> Deserialize<'de> for NotificationPreferencesFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(Vis)
    }
}
