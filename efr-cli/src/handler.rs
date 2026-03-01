use strum::VariantArray;

use crate::{
    config::EfrConfig,
    error::CliError,
    operations::{
        Operations, Services, authenticate_user, change_password, codes,
        create_payment_account_waiver, get_case, get_case_list, get_filing_list,
        get_notification_preferences, get_payment_account, get_payment_account_list,
        get_payment_account_type_list, get_policy, get_user, remove_payment_account,
        reset_password, self_resend_activation_email, update_notification_preferences, update_user,
    },
};

pub async fn handler() -> Result<(), CliError> {
    let config = EfrConfig::try_new()?;
    let client = reqwest::ClientBuilder::new().build()?;

    println!(
        "EFile Rust CLI: {:?}/{:?}",
        config.metadata.state, config.metadata.environment
    );

    let service = inquire::Select::new(
        "What Service Are You Looking To Use?",
        Services::VARIANTS.to_vec(),
    )
    .prompt()?;

    let operations = match service {
        Services::User => Operations::user(),
        Services::Firm => Operations::firm(),
        Services::CourtRecord => Operations::court_record(),
        Services::Policy => Operations::policy(),
        Services::FilingReview => Operations::filing_review(),
        Services::Codes => Operations::codes(),
    };

    let operation = inquire::Select::new(
        "What Operation Are You Looking To Preform?",
        operations.to_vec(),
    )
    .prompt()?;

    match operation {
        Operations::GetCaseList => get_case_list::handler(client, &config).await?,
        Operations::GetCase => get_case::handler(client, &config).await?,
        Operations::GetPolicy => get_policy::handler(client, &config).await?,
        Operations::AuthenticateUser => {
            authenticate_user::handler(client, &config).await?;
        }
        Operations::GetFilingList => get_filing_list::handler(client, &config).await?,
        Operations::GetPaymentAccount => {
            get_payment_account::handler(client, &config).await?;
        }
        Operations::GetPaymentAccountList => {
            get_payment_account_list::handler(client, &config).await?;
        }
        Operations::GetPaymentAccountTypeList => {
            get_payment_account_type_list::handler(client, &config).await?
        }
        Operations::CreatePaymentAccountWaiver => {
            create_payment_account_waiver::handler(client, &config).await?
        }
        Operations::RemovePaymentAccount => {
            remove_payment_account::handler(client, &config).await?
        }
        Operations::ChangePassword => change_password::handler(client, &config).await?,
        Operations::GetUser => get_user::handler(client, &config).await?,
        Operations::UpdateUser => update_user::handler(client, &config).await?,
        Operations::ResetPassword => reset_password::handler(client, &config).await?,
        Operations::GetNotificationPreferences => {
            get_notification_preferences::handler(client, &config).await?
        }
        Operations::UpdateNotificationPreferences => {
            update_notification_preferences::handler(client, &config).await?
        }
        Operations::SelfResendActivationEmail => {
            self_resend_activation_email::handler(client, &config).await?
        }
        Operations::LocationCodes => codes::location(client, &config).await?,
    }

    Ok(())
}
