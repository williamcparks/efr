# EFile Rust And Other Language Clients

[![crates.io](https://img.shields.io/crates/v/efr.svg)](https://crates.io/crates/efr)
[![docs.rs](https://docs.rs/efr/badge.svg)](https://docs.rs/efr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

# EFile CLI Demo

## Get Case List Conversion To JSON

<img width="1470" height="956" alt="Screenshot 2026-03-10 at 7 04 31 PM" src="https://github.com/user-attachments/assets/6474b4fc-2f70-4af8-ae5d-e4d787a1a23e" />

## Get Version Codes Conversion To Fast Rust Structs

https://github.com/user-attachments/assets/9ae3daf8-4f28-4d7d-9217-17f33aea71c1

# EFSP API Implementation Progress

Key

- ✅ = Impl.
- ❌ = Not Impl.
- JSON = XML response is converted to JSON

## States:

| State                | Stage | Production |
| -------------------- | ----- | ---------- |
| California           | ✅    | ✅         |
| District of Columbia | ✅    | ✅         |
| Georgia              | ✅    | ✅         |
| Idaho                | ✅    | ✅         |
| Illinois             | ✅    | ✅         |
| Indiana              | ✅    | ✅         |
| Louisiana            | ✅    | ✅         |
| Maine                | ✅    | ✅         |
| Maryland             | ✅    | ✅         |
| Massachusetts        | ✅    | ✅         |
| Minnesota            | ✅    | ✅         |
| Nevada               | ✅    | ✅         |
| New Mexico           | ✅    | ✅         |
| North Carolina       | ✅    | ✅         |
| North Dakota         | ✅    | ✅         |
| Ohio                 | ✅    | ✅         |
| Oklahoma             | ✅    | ✅         |
| Oregon               | ✅    | ✅         |
| Pennsylvania         | ✅    | ✅         |
| Rhode Island         | ✅    | ✅         |
| South Dakota         | ✅    | ✅         |
| Tennessee            | ✅    | ✅         |
| Texas                | ✅    | ✅         |
| Vermont              | ✅    | ✅         |
| Virginia             | ✅    | ✅         |
| Washington           | ✅    | ✅         |

## Codes

Here's the updated table with all code sets showing green:

| Operation                | Request | Response  |
| ------------------------ | ------- | --------- |
| `Location`               | ✅      | JSON      |
| `Versions`               | ✅      | ✅ & JSON |
| `Error`                  | ✅      | JSON      |
| `Country`                | ✅      | JSON      |
| `State`                  | ✅      | JSON      |
| `Filing Status`          | ✅      | JSON      |
| `Data Field`             | ✅      | JSON      |
| `Case Category`          | ✅      | JSON      |
| `Case Type`              | ✅      | JSON      |
| `Case Sub Type`          | ✅      | JSON      |
| `Party Type`             | ✅      | JSON      |
| `Filing`                 | ✅      | JSON      |
| `Filing Component`       | ✅      | JSON      |
| `Document Type`          | ✅      | JSON      |
| `File Type`              | ✅      | JSON      |
| `Optional Services`      | ✅      | JSON      |
| `Filer Type`             | ✅      | JSON      |
| `Procedure/Remedy`       | ✅      | JSON      |
| `Damage Amount`          | ✅      | JSON      |
| `Name Suffix`            | ✅      | JSON      |
| `Language`               | ✅      | JSON      |
| `Cross Reference`        | ✅      | JSON      |
| `Disclaimer Requirement` | ✅      | JSON      |
| `Motion Type`            | ✅      | JSON      |
| `Service Type`           | ✅      | JSON      |
| `Question`               | ✅      | JSON      |
| `Answer`                 | ✅      | JSON      |
| `Refund Reason`          | ✅      | JSON      |
| `Service Provider`       | ✅      | JSON      |
| `Notification Agency`    | ✅      | JSON      |
| `Arrest Location`        | ✅      | JSON      |
| `Bond`                   | ✅      | JSON      |
| `Charge Phase`           | ✅      | JSON      |
| `Degree`                 | ✅      | JSON      |
| `Driver License Type`    | ✅      | JSON      |
| `Ethnicity`              | ✅      | JSON      |
| `Eye Color`              | ✅      | JSON      |
| `General Offense`        | ✅      | JSON      |
| `Hair Color`             | ✅      | JSON      |
| `Law Enforcement Unit`   | ✅      | JSON      |
| `Physical Feature`       | ✅      | JSON      |
| `Race`                   | ✅      | JSON      |
| `Statute`                | ✅      | JSON      |
| `Statute Type`           | ✅      | JSON      |
| `Citation Jurisdiction`  | ✅      | JSON      |
| `Vehicle Color`          | ✅      | JSON      |
| `Vehicle Make`           | ✅      | JSON      |
| `Vehicle Type`           | ✅      | JSON      |

## User

| Operation                       | Request | Response  |
| ------------------------------- | ------- | --------- |
| `AuthenticateUser`              | ✅      | ✅ & JSON |
| `ChangePassword`                | ✅      | JSON      |
| `GetUser`                       | ✅      | JSON      |
| `UpdateUser`                    | ✅      | JSON      |
| `ResetPassword`                 | ✅      | JSON      |
| `GetNotificationPreferences`    | ✅      | JSON      |
| `UpdateNotificationPreferences` | ✅      | JSON      |
| `ResendActivationEmail` (self)  | ✅      | JSON      |

---

## Firm

| Operation                        | Request | Response |
| -------------------------------- | ------- | -------- |
| `GetFirm`                        | ❌      | JSON     |
| `UpdateFirm`                     | ❌      | JSON     |
| `RegisterUser`                   | ❌      | JSON     |
| `GetUser` (firm)                 | ❌      | JSON     |
| `GetUserList`                    | ❌      | JSON     |
| `RemoveUser`                     | ❌      | JSON     |
| `ResendActivationEmail`          | ❌      | JSON     |
| `ResetUserPassword`              | ❌      | JSON     |
| `AddUserRole`                    | ❌      | JSON     |
| `RemoveUserRole`                 | ❌      | JSON     |
| `CreatePaymentAccount`           | ✅      | JSON     |
| `GetPaymentAccount`              | ✅      | JSON     |
| `GetPaymentAccountList`          | ✅      | JSON     |
| `GetPaymentAccountTypeList`      | ✅      | JSON     |
| `UpdatePaymentAccount`           | ✅      | JSON     |
| `RemovePaymentAccount`           | ✅      | JSON     |
| `CreatePaymentAccountWaiver`     | ✅      | JSON     |
| `CreateGlobalPaymentAccount`     | ❌      | JSON     |
| `GetGlobalPaymentAccount`        | ❌      | JSON     |
| `GetGlobalPaymentAccountList`    | ❌      | JSON     |
| `UpdateGlobalPaymentAccount`     | ❌      | JSON     |
| `RemoveGlobalPaymentAccount`     | ❌      | JSON     |
| `GetVitalChekPaymentAccountId`   | ❌      | JSON     |
| `CreateAttorney`                 | ❌      | JSON     |
| `GetAttorney`                    | ❌      | JSON     |
| `GetAttorneyList`                | ❌      | JSON     |
| `UpdateAttorney`                 | ❌      | JSON     |
| `RemoveAttorney`                 | ❌      | JSON     |
| `CreateServiceContact`           | ❌      | JSON     |
| `GetServiceContact`              | ❌      | JSON     |
| `GetServiceContactList`          | ❌      | JSON     |
| `UpdateServiceContact`           | ❌      | JSON     |
| `RemoveServiceContact`           | ❌      | JSON     |
| `AttachServiceContact`           | ❌      | JSON     |
| `DetachServiceContact`           | ❌      | JSON     |
| `GetPublicList`                  | ❌      | JSON     |
| `GetNotificationPreferencesList` | ❌      | JSON     |

---

## FilingReview

| Operation                        | Request              | Response |
| -------------------------------- | -------------------- | -------- |
| `GetFilingList`                  | ✅                   | JSON     |
| `GetFilingDetails`               | ❌                   | JSON     |
| `GetFilingStatus`                | ❌                   | JSON     |
| `GetFilingService`               | ❌                   | JSON     |
| `GetFeesCalculation`             | ❌                   | JSON     |
| `CancelFiling`                   | ❌                   | JSON     |
| `ReviewFiling`                   | ✍️(Work In Progress) | JSON     |
| `GetServiceTypes`                | ❌                   | JSON     |
| `GetPolicy`                      | ✅                   | JSON     |
| `GetBatchList` _(Tyler ext.)_    | ❌                   | JSON     |
| `GetBatchDetails` _(Tyler ext.)_ | ❌                   | JSON     |

---

## FilingAssembly _(inbound callbacks)_

| Operation                            | Handler | Response |
| ------------------------------------ | ------- | -------- |
| `NotifyFilingReviewComplete`         | ❌      | JSON     |
| `NotifyEvent`                        | ❌      | JSON     |
| `NotifyServiceComplete`              | ❌      | JSON     |
| `NotifyBatchComplete` _(Tyler ext.)_ | ❌      | JSON     |

---

## Court Record

| Operation                      | Request | Response |
| ------------------------------ | ------- | -------- |
| `GetCase`                      | ✅      | JSON     |
| `GetCaseList`                  | ✅      | JSON     |
| `GetServiceAttachCaseList`     | ❌      | JSON     |
| `GetServiceInformation`        | ❌      | JSON     |
| `GetServiceInformationHistory` | ❌      | JSON     |

---

## Serving

| Operation     | Request | Response |
| ------------- | ------- | -------- |
| `ServeFiling` | ❌      | JSON     |

---

## Court Scheduling

| Operation          | Request | Response |
| ------------------ | ------- | -------- |
| `GetReturnDate`    | ❌      | JSON     |
| `ReserveCourtDate` | ❌      | JSON     |

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
