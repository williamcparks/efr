use bitflags::bitflags;
use serde::{Deserialize, Serialize};

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
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
