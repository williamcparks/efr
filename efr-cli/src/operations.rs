use efr::api::{Environment, Metadata, State};
use strum::{Display, VariantArray};

#[derive(Clone, Display, VariantArray)]
pub enum Operations {
    #[strum(to_string = "Get Case List")]
    GetCaseList,

    #[strum(to_string = "Get Case")]
    GetCase,

    #[strum(to_string = "Get Policy")]
    GetPolicy,

    #[strum(to_string = "Authenticate User")]
    AuthenticateUser,

    #[strum(to_string = "Change Password")]
    ChangePassword,

    #[strum(to_string = "Get User")]
    GetUser,

    #[strum(to_string = "Update User")]
    UpdateUser,

    #[strum(to_string = "Reset Password (uses email)")]
    ResetPassword,

    #[strum(to_string = "Get Notification Preferences")]
    GetNotificationPreferences,

    #[strum(to_string = "Update Notification Preferences")]
    UpdateNotificationPreferences,

    #[strum(to_string = "Self Resend Activation Email (uses email)")]
    SelfResendActivationEmail,
}

pub mod authenticate_user;
pub mod change_password;
pub mod error;
pub mod get_case;
pub mod get_case_list;
pub mod get_notification_preferences;
pub mod get_policy;
pub mod get_user;
pub mod post;
pub mod reset_password;
pub mod self_resend_activation_email;
pub mod update_notification_preferences;
pub mod update_user;

pub const METADATA: Metadata = Metadata {
    state: State::Texas,
    environment: Environment::Stage,
};
