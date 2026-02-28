use efr::api::{Environment, Metadata, State};
use efr_macros::SubClass;
use strum::{Display, VariantArray};

#[derive(Clone, Display, VariantArray)]
pub enum Services {
    User,
    Firm,
    #[strum(to_string = "Court Record")]
    CourtRecord,
    #[strum(to_string = "Filing Review")]
    FilingReview,
    Policy,
}

#[derive(Clone, Display, SubClass)]
#[subclass(user, firm, court_record, filing_review, policy)]
pub enum Operations {
    #[subclass(court_record)]
    #[strum(to_string = "Get Case List")]
    GetCaseList,

    #[subclass(court_record)]
    #[strum(to_string = "Get Case")]
    GetCase,

    #[subclass(policy)]
    #[strum(to_string = "Get Policy")]
    GetPolicy,

    #[subclass(user)]
    #[strum(to_string = "Authenticate User")]
    AuthenticateUser,

    #[subclass(filing_review)]
    #[strum(to_string = "Get Filing List")]
    GetFilingList,

    #[subclass(firm)]
    #[strum(to_string = "Get Payment Account")]
    GetPaymentAccount,

    #[subclass(firm)]
    #[strum(to_string = "Get Payment Account List")]
    GetPaymentAccountList,

    #[subclass(firm)]
    #[strum(to_string = "Get Payment Account Type List (fetches codes)")]
    GetPaymentAccountTypeList,

    #[subclass(firm)]
    #[strum(to_string = "Create Payment Account Waiver")]
    CreatePaymentAccountWaiver,

    #[subclass(firm)]
    #[strum(to_string = "Remove Payment Account")]
    RemovePaymentAccount,

    #[subclass(user)]
    #[strum(to_string = "Change Password")]
    ChangePassword,

    #[subclass(user)]
    #[strum(to_string = "Get User")]
    GetUser,

    #[subclass(user)]
    #[strum(to_string = "Update User")]
    UpdateUser,

    #[subclass(user)]
    #[strum(to_string = "Reset Password (uses email)")]
    ResetPassword,

    #[subclass(user)]
    #[strum(to_string = "Get Notification Preferences")]
    GetNotificationPreferences,

    #[subclass(user)]
    #[strum(to_string = "Update Notification Preferences")]
    UpdateNotificationPreferences,

    #[subclass(user)]
    #[strum(to_string = "Self Resend Activation Email (uses email)")]
    SelfResendActivationEmail,
}

pub mod authenticate_user;
pub mod change_password;
pub mod create_payment_account_waiver;
pub mod error;
pub mod get_case;
pub mod get_case_list;
pub mod get_filing_list;
pub mod get_notification_preferences;
pub mod get_payment_account;
pub mod get_payment_account_list;
pub mod get_payment_account_type_list;
pub mod get_policy;
pub mod get_user;
pub mod inquire_helpers;
pub mod post;
pub mod remove_payment_account;
pub mod reset_password;
pub mod self_resend_activation_email;
pub mod update_notification_preferences;
pub mod update_user;

pub const METADATA: Metadata = Metadata {
    state: State::Texas,
    environment: Environment::Stage,
};
