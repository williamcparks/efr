# EFile Rust And Other Language Clients

[![crates.io](https://img.shields.io/crates/v/efr.svg)](https://crates.io/crates/efr)
[![docs.rs](https://docs.rs/efr/badge.svg)](https://docs.rs/efr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

# EFSP API Implementation Progress

## States:

| State                | Stage | Production |
| -------------------- | ----- | ---------- |
| California           | ❌    | ❌         |
| District of Columbia | ❌    | ❌         |
| Georgia              | ❌    | ❌         |
| Idaho                | ❌    | ❌         |
| Illinois             | ❌    | ❌         |
| Indiana              | ❌    | ❌         |
| Louisiana            | ❌    | ❌         |
| Maine                | ❌    | ❌         |
| Maryland             | ❌    | ❌         |
| Massachusetts        | ❌    | ❌         |
| Minnesota            | ❌    | ❌         |
| Nevada               | ❌    | ❌         |
| New Mexico           | ❌    | ❌         |
| North Carolina       | ❌    | ❌         |
| North Dakota         | ❌    | ❌         |
| Ohio                 | ❌    | ❌         |
| Oklahoma             | ❌    | ❌         |
| Oregon               | ❌    | ❌         |
| Pennsylvania         | ❌    | ❌         |
| Rhode Island         | ❌    | ❌         |
| South Dakota         | ❌    | ❌         |
| Tennessee            | ❌    | ❌         |
| Texas                | ✅    | ❌         |
| Vermont              | ❌    | ❌         |
| Virginia             | ❌    | ❌         |
| Washington           | ❌    | ❌         |

## EFMUserService

| Operation                       | Request | Response |
| ------------------------------- | ------- | -------- |
| `AuthenticateUser`              | ✅      | ✅       |
| `ChangePassword`                | ✅      | ❌       |
| `GetUser`                       | ✅      | ❌       |
| `GetUserSelf`                   | ❌      | ❌       |
| `UpdateUser`                    | ✅      | ❌       |
| `ResetPassword`                 | ✅      | ❌       |
| `GetNotificationPreferences`    | ✅      | ❌       |
| `UpdateNotificationPreferences` | ✅      | ❌       |
| `ResendActivationEmail` (self)  | ✅      | ❌       |

---

## EFMFirmService

| Operation                        | Request | Response |
| -------------------------------- | ------- | -------- |
| `GetFirm`                        | ❌      | ❌       |
| `UpdateFirm`                     | ❌      | ❌       |
| `RegisterUser`                   | ❌      | ❌       |
| `GetUser` (firm)                 | ❌      | ❌       |
| `GetUserList`                    | ❌      | ❌       |
| `RemoveUser`                     | ❌      | ❌       |
| `ResendActivationEmail`          | ❌      | ❌       |
| `ResetUserPassword`              | ❌      | ❌       |
| `AddUserRole`                    | ❌      | ❌       |
| `RemoveUserRole`                 | ❌      | ❌       |
| `CreatePaymentAccount`           | ❌      | ❌       |
| `GetPaymentAccount`              | ✅      | ❌       |
| `GetPaymentAccountList`          | ✅      | ❌       |
| `GetPaymentAccountTypeList`      | ✅      | ❌       |
| `UpdatePaymentAccount`           | ❌      | ❌       |
| `RemovePaymentAccount`           | ✅      | ❌       |
| `CreatePaymentAccountWaiver`     | ✅      | ❌       |
| `CreateGlobalPaymentAccount`     | ❌      | ❌       |
| `GetGlobalPaymentAccount`        | ❌      | ❌       |
| `GetGlobalPaymentAccountList`    | ❌      | ❌       |
| `UpdateGlobalPaymentAccount`     | ❌      | ❌       |
| `RemoveGlobalPaymentAccount`     | ❌      | ❌       |
| `GetVitalChekPaymentAccountId`   | ❌      | ❌       |
| `CreateAttorney`                 | ❌      | ❌       |
| `GetAttorney`                    | ❌      | ❌       |
| `GetAttorneyList`                | ❌      | ❌       |
| `UpdateAttorney`                 | ❌      | ❌       |
| `RemoveAttorney`                 | ❌      | ❌       |
| `CreateServiceContact`           | ❌      | ❌       |
| `GetServiceContact`              | ❌      | ❌       |
| `GetServiceContactList`          | ❌      | ❌       |
| `UpdateServiceContact`           | ❌      | ❌       |
| `RemoveServiceContact`           | ❌      | ❌       |
| `AttachServiceContact`           | ❌      | ❌       |
| `DetachServiceContact`           | ❌      | ❌       |
| `GetPublicList`                  | ❌      | ❌       |
| `GetNotificationPreferencesList` | ❌      | ❌       |

---

## FilingReviewMDEService

| Operation                        | Request | Response |
| -------------------------------- | ------- | -------- |
| `GetFilingList`                  | ✅      | ❌       |
| `GetFilingDetails`               | ❌      | ❌       |
| `GetFilingStatus`                | ❌      | ❌       |
| `GetFilingService`               | ❌      | ❌       |
| `GetFeesCalculation`             | ❌      | ❌       |
| `CancelFiling`                   | ❌      | ❌       |
| `ReviewFiling`                   | ❌      | ❌       |
| `GetServiceTypes`                | ❌      | ❌       |
| `GetPolicy`                      | ✅      | ❌       |
| `GetBatchList` _(Tyler ext.)_    | ❌      | ❌       |
| `GetBatchDetails` _(Tyler ext.)_ | ❌      | ❌       |

---

## FilingAssemblyMDEService _(inbound callbacks)_

| Operation                            | Handler | Response |
| ------------------------------------ | ------- | -------- |
| `NotifyFilingReviewComplete`         | ❌      | ❌       |
| `NotifyEvent`                        | ❌      | ❌       |
| `NotifyServiceComplete`              | ❌      | ❌       |
| `NotifyBatchComplete` _(Tyler ext.)_ | ❌      | ❌       |

---

## CourtRecordMDEService

| Operation                      | Request | Response |
| ------------------------------ | ------- | -------- |
| `GetCase`                      | ✅      | ⏳       |
| `GetCaseList`                  | ✅      | ⏳       |
| `GetServiceAttachCaseList`     | ❌      | ❌       |
| `GetServiceInformation`        | ❌      | ❌       |
| `GetServiceInformationHistory` | ❌      | ❌       |

---

## ServiceMDEService

| Operation     | Request | Response |
| ------------- | ------- | -------- |
| `ServeFiling` | ❌      | ❌       |

---

## CourtSchedulingMDEService

| Operation          | Request | Response |
| ------------------ | ------- | -------- |
| `GetReturnDate`    | ❌      | ❌       |
| `ReserveCourtDate` | ❌      | ❌       |

## CLI

See the [CLI documentation](cli.md) for command-line usage.

## Other Languages

Currently just a proof of concept, all can send an authenticate user message and recieve a correct response from the EFM
other features are on hold till rust client is complete

- [ ] - Python [folder](other-languages/python)
- [ ] - Typescript [folder](other-languages/typescript)
- [ ] - AWS Lambda Rust Proxy Demo [folder](other-languages/rust-lambda)

## Links

- [crates.io](https://crates.io/crates/efr)
- [docs.rs](https://docs.rs/efr)
- [Repository](https://github.com/williamcparks/efr)

## License

This project is licensed under the [MIT License](LICENSE).
