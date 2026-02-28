use efr_macros::xml;
use serde::{Deserialize, Serialize};

use crate::{
    api::{IntBool, SecureEfrRequest},
    user_service::NotificationPreferencesFlags,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateNotificationPreferencesRequest<'a> {
    pub email: &'a str,
    pub password_hash: &'a str,

    pub flags: NotificationPreferencesFlags,
}

impl<'a> SecureEfrRequest for UpdateNotificationPreferencesRequest<'a> {
    const SOAP_ACTION: &'static str =
        "urn:tyler:efm:services/IEfmUserService/UpdateNotificationPreferences";

    fn email(&self) -> &str {
        self.email
    }

    fn password_hash(&self) -> &str {
        self.password_hash
    }
}

xml! {
    impl<'a> Xml for UpdateNotificationPreferencesRequest<'a> {
        @ns {
            common = "urn:tyler:efm:services:schema:Common";
            tyler = "urn:tyler:efm:services";
        }

        tyler:UpdateNotificationPreferences {
            tyler:UpdateNotificationPreferencesRequest {
                common:Notification {
                    common:Code { "ACCEPTED" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::ACCEPTED))) }
                }
                common:Notification {
                    common:Code { "REJECTED" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::REJECTED))) }
                }
                common:Notification {
                    common:Code { "SUBMITTED" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::SUBMITTED))) }
                }
                common:Notification {
                    common:Code { "SERVICEUNDELIVERABLE" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::SERVICEUNDELIVERABLE))) }
                }
                common:Notification {
                    common:Code { "SUBMISSIONFAILED" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::SUBMISSIONFAILED))) }
                }
                common:Notification {
                    common:Code { "RECEIPTED" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::RECEIPTED))) }
                }
                common:Notification {
                    common:Code { "RETURNFORCORRECTION" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::RETURNFORCORRECTION))) }
                }
                common:Notification {
                    common:Code { "ACCOUNTLOCKED" }
                    common:IsActive { (&IntBool(self.flags.contains(NotificationPreferencesFlags::ACCOUNTLOCKED))) }
                }
            }
        }
    }
}
