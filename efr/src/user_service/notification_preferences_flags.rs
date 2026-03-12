use bitflags::bitflags;
use serde::{Deserialize, Serialize};

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

impl<'de> Deserialize<'de> for NotificationPreferencesFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u8::deserialize(deserializer)?;
        Ok(Self::from_bits_truncate(bits))
    }
}
