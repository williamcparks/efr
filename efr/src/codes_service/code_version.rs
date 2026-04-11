use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{Display, EnumString, VariantArray};

use crate::codes_service::EfrCodesError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeVersion {
    pub location: Box<str>,
    pub codelist: CodeList,
    pub version: Box<str>,
}

impl CodeVersion {
    pub fn from_value(value: Value) -> Result<Self, EfrCodesError> {
        Ok(serde_json::from_value(value)?)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    Display,
    VariantArray,
    EnumString,
    Deserialize,
    Serialize,
)]
pub enum CodeList {
    // ── System-wide ──────────────────────────────────────────────────────────
    #[strum(serialize = "countrycodes.zip")]
    Country,
    #[strum(serialize = "datafieldconfigcodes.zip")]
    DataFieldConfig,
    #[strum(serialize = "errorcodes.zip")]
    Error,
    #[strum(serialize = "filingstatuscodes.zip")]
    FilingStatus,
    #[strum(serialize = "languagecodes.zip")]
    Language,
    #[strum(serialize = "locations.zip")]
    Locations,
    #[strum(serialize = "statecodes.zip")]
    State,

    // ── Court-specific ───────────────────────────────────────────────────────
    #[strum(serialize = "answercodes.zip")]
    Answer,
    #[strum(serialize = "appellatelowercourts.zip")]
    AppellateLowerCourts,
    #[strum(serialize = "casecategorycodes.zip")]
    CaseCategory,
    #[strum(serialize = "casesubtypecodes.zip")]
    CaseSubType,
    #[strum(serialize = "casetypecodes.zip")]
    CaseType,
    #[strum(serialize = "crossreferencecodes.zip")]
    CrossReference,
    #[strum(serialize = "damageamountcodes.zip")]
    DamageAmount,
    #[strum(serialize = "disclaimerrequirementcodes.zip")]
    DisclaimerRequirement,
    #[strum(serialize = "documenttypecodes.zip")]
    DocumentType,
    #[strum(serialize = "filertypecodes.zip")]
    FilerType,
    #[strum(serialize = "filetypecodes.zip")]
    FileType,
    #[strum(serialize = "filingcodes.zip")]
    Filing,
    #[strum(serialize = "filingcomponentcodes.zip")]
    FilingComponent,
    #[strum(serialize = "hearinglocationcodes.zip")]
    HearingLocation,
    #[strum(serialize = "judicialofficercodes.zip")]
    JudicialOfficer,
    #[strum(serialize = "motiontypecodes.zip")]
    MotionType,
    #[strum(serialize = "namesuffixcodes.zip")]
    NameSuffix,
    #[strum(serialize = "notificationagencycodes.zip")]
    NotificationAgency,
    #[strum(serialize = "optionalservicescodes.zip")]
    OptionalServices,
    #[strum(serialize = "partytypecodes.zip")]
    PartyType,
    #[strum(serialize = "procedureremedycodes.zip")]
    ProcedureRemedy,
    #[strum(serialize = "questioncodes.zip")]
    Question,
    #[strum(serialize = "refundreasoncodes.zip")]
    RefundReason,
    #[strum(serialize = "repcapcodes.zip")]
    RepCap,
    #[strum(serialize = "serviceprovidercodes.zip")]
    ServiceProvider,
    #[strum(serialize = "servicetypecodes.zip")]
    ServiceType,

    // ── Criminal initiation ──────────────────────────────────────────────────
    #[strum(serialize = "arrestlocationcodes.zip")]
    ArrestLocation,
    #[strum(serialize = "bondcodes.zip")]
    Bond,
    #[strum(serialize = "chargephasecodes.zip")]
    ChargePhase,
    #[strum(serialize = "citationjurisdictioncodes.zip")]
    CitationJurisdiction,
    #[strum(serialize = "degreecodes.zip")]
    Degree,
    #[strum(serialize = "driverlicensetypecodes.zip")]
    DriverLicenseType,
    #[strum(serialize = "ethnicitycodes.zip")]
    Ethnicity,
    #[strum(serialize = "eyecolorcodes.zip")]
    EyeColor,
    #[strum(serialize = "generaloffensecodes.zip")]
    GeneralOffense,
    #[strum(serialize = "haircolorcodes.zip")]
    HairColor,
    #[strum(serialize = "lawenforcementunitcodes.zip")]
    LawEnforcementUnit,
    #[strum(serialize = "physicalfeaturecodes.zip")]
    PhysicalFeature,
    #[strum(serialize = "racecodes.zip")]
    Race,
    #[strum(serialize = "statutecodes.zip")]
    Statute,
    #[strum(serialize = "statutetypecodes.zip")]
    StatuteType,
    #[strum(serialize = "vehiclecolorcodes.zip")]
    VehicleColor,
    #[strum(serialize = "vehiclemakecodes.zip")]
    VehicleMake,
    #[strum(serialize = "vehicletypecodes.zip")]
    VehicleType,
}

impl CodeList {
    pub const fn all() -> &'static [Self] {
        Self::VARIANTS
    }

    pub fn name(&self) -> &'static str {
        match self {
            // ── System-wide ──────────────────────────────────────────────────
            Self::Country => "countrycodes",
            Self::DataFieldConfig => "datafieldconfigcodes",
            Self::Error => "errorcodes",
            Self::FilingStatus => "filingstatuscodes",
            Self::Language => "languagecodes",
            Self::Locations => "locations",
            Self::State => "statecodes",

            // ── Court-specific ───────────────────────────────────────────────
            Self::Answer => "answercodes",
            Self::AppellateLowerCourts => "appellatelowercourts",
            Self::CaseCategory => "casecategorycodes",
            Self::CaseSubType => "casesubtypecodes",
            Self::CaseType => "casetypecodes",
            Self::CrossReference => "crossreferencecodes",
            Self::DamageAmount => "damageamountcodes",
            Self::DisclaimerRequirement => "disclaimerrequirementcodes",
            Self::DocumentType => "documenttypecodes",
            Self::FilerType => "filertypecodes",
            Self::FileType => "filetypecodes",
            Self::Filing => "filingcodes",
            Self::FilingComponent => "filingcomponentcodes",
            Self::HearingLocation => "hearinglocationcodes",
            Self::JudicialOfficer => "judicialofficercodes",
            Self::MotionType => "motiontypecodes",
            Self::NameSuffix => "namesuffixcodes",
            Self::NotificationAgency => "notificationagencycodes",
            Self::OptionalServices => "optionalservicescodes",
            Self::PartyType => "partytypecodes",
            Self::ProcedureRemedy => "procedureremedycodes",
            Self::Question => "questioncodes",
            Self::RefundReason => "refundreasoncodes",
            Self::RepCap => "repcapcodes",
            Self::ServiceProvider => "serviceprovidercodes",
            Self::ServiceType => "servicetypecodes",

            // ── Criminal initiation ──────────────────────────────────────────
            Self::ArrestLocation => "arrestlocationcodes",
            Self::Bond => "bondcodes",
            Self::ChargePhase => "chargephasecodes",
            Self::CitationJurisdiction => "citationjurisdictioncodes",
            Self::Degree => "degreecodes",
            Self::DriverLicenseType => "driverlicensetypecodes",
            Self::Ethnicity => "ethnicitycodes",
            Self::EyeColor => "eyecolorcodes",
            Self::GeneralOffense => "generaloffensecodes",
            Self::HairColor => "haircolorcodes",
            Self::LawEnforcementUnit => "lawenforcementunitcodes",
            Self::PhysicalFeature => "physicalfeaturecodes",
            Self::Race => "racecodes",
            Self::Statute => "statutecodes",
            Self::StatuteType => "statutetypecodes",
            Self::VehicleColor => "vehiclecolorcodes",
            Self::VehicleMake => "vehiclemakecodes",
            Self::VehicleType => "vehicletypecodes",
        }
    }

    #[cfg(feature = "metadata")]
    pub fn url(
        &self,
        metadata: &crate::api::Metadata,
        jurisdiction: Option<&str>,
    ) -> Cow<'static, str> {
        let jurisdiction = jurisdiction.unwrap_or_default();
        match self {
            // ── System-wide ──────────────────────────────────────────────────────────
            Self::Country => Cow::Borrowed(metadata.country_url()),
            Self::DataFieldConfig => Cow::Borrowed(metadata.data_field_config_url()),
            Self::Error => Cow::Borrowed(metadata.error_url()),
            Self::FilingStatus => Cow::Borrowed(metadata.filing_status_url()),
            Self::Language => Cow::Borrowed(metadata.language_url()),
            Self::Locations => Cow::Borrowed(metadata.locations_url()),
            Self::State => Cow::Borrowed(metadata.state_url()),
            // ── Court-specific ───────────────────────────────────────────────────────
            Self::Answer => Cow::Owned(metadata.answer_url(jurisdiction)),
            Self::AppellateLowerCourts => {
                Cow::Owned(metadata.appellate_lower_courts_url(jurisdiction))
            }
            Self::CaseCategory => Cow::Owned(metadata.case_category_url(jurisdiction)),
            Self::CaseSubType => Cow::Owned(metadata.case_sub_type_url(jurisdiction)),
            Self::CaseType => Cow::Owned(metadata.case_type_url(jurisdiction)),
            Self::CrossReference => Cow::Owned(metadata.cross_reference_url(jurisdiction)),
            Self::DamageAmount => Cow::Owned(metadata.damage_amount_url(jurisdiction)),
            Self::DisclaimerRequirement => {
                Cow::Owned(metadata.disclaimer_requirement_url(jurisdiction))
            }
            Self::DocumentType => Cow::Owned(metadata.document_type_url(jurisdiction)),
            Self::FilerType => Cow::Owned(metadata.filer_type_url(jurisdiction)),
            Self::FileType => Cow::Owned(metadata.file_type_url(jurisdiction)),
            Self::Filing => Cow::Owned(metadata.filing_url(jurisdiction)),
            Self::FilingComponent => Cow::Owned(metadata.filing_component_url(jurisdiction)),
            Self::HearingLocation => Cow::Owned(metadata.hearing_location_url(jurisdiction)),
            Self::JudicialOfficer => Cow::Owned(metadata.judicial_officer_url(jurisdiction)),
            Self::MotionType => Cow::Owned(metadata.motion_type_url(jurisdiction)),
            Self::NameSuffix => Cow::Owned(metadata.name_suffix_url(jurisdiction)),
            Self::NotificationAgency => Cow::Owned(metadata.notification_agency_url(jurisdiction)),
            Self::OptionalServices => Cow::Owned(metadata.optional_services_url(jurisdiction)),
            Self::PartyType => Cow::Owned(metadata.party_type_url(jurisdiction)),
            Self::ProcedureRemedy => Cow::Owned(metadata.procedure_remedy_url(jurisdiction)),
            Self::Question => Cow::Owned(metadata.question_url(jurisdiction)),
            Self::RefundReason => Cow::Owned(metadata.refund_reason_url(jurisdiction)),
            Self::RepCap => Cow::Owned(metadata.rep_cap_url(jurisdiction)),
            Self::ServiceProvider => Cow::Owned(metadata.service_provider_url(jurisdiction)),
            Self::ServiceType => Cow::Owned(metadata.service_type_url(jurisdiction)),
            // ── Criminal initiation ──────────────────────────────────────────────────
            Self::ArrestLocation => Cow::Owned(metadata.arrest_location_url(jurisdiction)),
            Self::Bond => Cow::Owned(metadata.bond_url(jurisdiction)),
            Self::ChargePhase => Cow::Owned(metadata.charge_phase_url(jurisdiction)),
            Self::CitationJurisdiction => {
                Cow::Owned(metadata.citation_jurisdiction_url(jurisdiction))
            }
            Self::Degree => Cow::Owned(metadata.degree_url(jurisdiction)),
            Self::DriverLicenseType => Cow::Owned(metadata.driver_license_type_url(jurisdiction)),
            Self::Ethnicity => Cow::Owned(metadata.ethnicity_url(jurisdiction)),
            Self::EyeColor => Cow::Owned(metadata.eye_color_url(jurisdiction)),
            Self::GeneralOffense => Cow::Owned(metadata.general_offense_url(jurisdiction)),
            Self::HairColor => Cow::Owned(metadata.hair_color_url(jurisdiction)),
            Self::LawEnforcementUnit => Cow::Owned(metadata.law_enforcement_unit_url(jurisdiction)),
            Self::PhysicalFeature => Cow::Owned(metadata.physical_feature_url(jurisdiction)),
            Self::Race => Cow::Owned(metadata.race_url(jurisdiction)),
            Self::Statute => Cow::Owned(metadata.statute_url(jurisdiction)),
            Self::StatuteType => Cow::Owned(metadata.statute_type_url(jurisdiction)),
            Self::VehicleColor => Cow::Owned(metadata.vehicle_color_url(jurisdiction)),
            Self::VehicleMake => Cow::Owned(metadata.vehicle_make_url(jurisdiction)),
            Self::VehicleType => Cow::Owned(metadata.vehicle_type_url(jurisdiction)),
        }
    }

    pub const fn requires_jurisdiction(&self) -> bool {
        match self {
            // ── System-wide ──────────────────────────────────────────────────────────
            Self::Country
            | Self::DataFieldConfig
            | Self::Error
            | Self::FilingStatus
            | Self::Language
            | Self::Locations
            | Self::State => false,
            // ── Court-specific & Criminal initiation ─────────────────────────────────
            _ => true,
        }
    }
}
