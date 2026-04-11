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
    #[serde(alias = "codeversions.zip")]
    #[strum(serialize = "codeversions.zip")]
    Versions,
    #[serde(alias = "countrycodes.zip")]
    #[strum(serialize = "countrycodes.zip")]
    Country,
    #[serde(alias = "datafieldconfigcodes.zip")]
    #[strum(serialize = "datafieldconfigcodes.zip")]
    DataFieldConfig,
    #[serde(alias = "errorcodes.zip")]
    #[strum(serialize = "errorcodes.zip")]
    Error,
    #[serde(alias = "filingstatuscodes.zip")]
    #[strum(serialize = "filingstatuscodes.zip")]
    FilingStatus,
    #[serde(alias = "languagecodes.zip")]
    #[strum(serialize = "languagecodes.zip")]
    Language,
    #[serde(alias = "locations.zip")]
    #[strum(serialize = "locations.zip")]
    Locations,
    #[serde(alias = "statecodes.zip")]
    #[strum(serialize = "statecodes.zip")]
    State,

    // ── Court-specific ───────────────────────────────────────────────────────
    #[serde(alias = "answercodes.zip")]
    #[strum(serialize = "answercodes.zip")]
    Answer,
    #[serde(alias = "appellatelowercourts.zip")]
    #[strum(serialize = "appellatelowercourts.zip")]
    AppellateLowerCourts,
    #[serde(alias = "casecategorycodes.zip")]
    #[strum(serialize = "casecategorycodes.zip")]
    CaseCategory,
    #[serde(alias = "casesubtypecodes.zip")]
    #[strum(serialize = "casesubtypecodes.zip")]
    CaseSubType,
    #[serde(alias = "casetypecodes.zip")]
    #[strum(serialize = "casetypecodes.zip")]
    CaseType,
    #[serde(alias = "crossreferencecodes.zip")]
    #[strum(serialize = "crossreferencecodes.zip")]
    CrossReference,
    #[serde(alias = "damageamountcodes.zip")]
    #[strum(serialize = "damageamountcodes.zip")]
    DamageAmount,
    #[serde(alias = "disclaimerrequirementcodes.zip")]
    #[strum(serialize = "disclaimerrequirementcodes.zip")]
    DisclaimerRequirement,
    #[serde(alias = "documenttypecodes.zip")]
    #[strum(serialize = "documenttypecodes.zip")]
    DocumentType,
    #[serde(alias = "filertypecodes.zip")]
    #[strum(serialize = "filertypecodes.zip")]
    FilerType,
    #[serde(alias = "filetypecodes.zip")]
    #[strum(serialize = "filetypecodes.zip")]
    FileType,
    #[serde(alias = "filingcodes.zip")]
    #[strum(serialize = "filingcodes.zip")]
    Filing,
    #[serde(alias = "filingcomponentcodes.zip")]
    #[strum(serialize = "filingcomponentcodes.zip")]
    FilingComponent,
    #[serde(alias = "hearinglocationcodes.zip")]
    #[strum(serialize = "hearinglocationcodes.zip")]
    HearingLocation,
    #[serde(alias = "judicialofficercodes.zip")]
    #[strum(serialize = "judicialofficercodes.zip")]
    JudicialOfficer,
    #[serde(alias = "motiontypecodes.zip")]
    #[strum(serialize = "motiontypecodes.zip")]
    MotionType,
    #[serde(alias = "namesuffixcodes.zip")]
    #[strum(serialize = "namesuffixcodes.zip")]
    NameSuffix,
    #[serde(alias = "notificationagencycodes.zip")]
    #[strum(serialize = "notificationagencycodes.zip")]
    NotificationAgency,
    #[serde(alias = "optionalservicescodes.zip")]
    #[strum(serialize = "optionalservicescodes.zip")]
    OptionalServices,
    #[serde(alias = "partytypecodes.zip")]
    #[strum(serialize = "partytypecodes.zip")]
    PartyType,
    #[serde(alias = "procedureremedycodes.zip")]
    #[strum(serialize = "procedureremedycodes.zip")]
    ProcedureRemedy,
    #[serde(alias = "questioncodes.zip")]
    #[strum(serialize = "questioncodes.zip")]
    Question,
    #[serde(alias = "refundreasoncodes.zip")]
    #[strum(serialize = "refundreasoncodes.zip")]
    RefundReason,
    #[serde(alias = "repcapcodes.zip")]
    #[strum(serialize = "repcapcodes.zip")]
    RepCap,
    #[serde(alias = "serviceprovidercodes.zip")]
    #[strum(serialize = "serviceprovidercodes.zip")]
    ServiceProvider,
    #[serde(alias = "servicetypecodes.zip")]
    #[strum(serialize = "servicetypecodes.zip")]
    ServiceType,

    // ── Criminal initiation ──────────────────────────────────────────────────
    #[serde(alias = "arrestlocationcodes.zip")]
    #[strum(serialize = "arrestlocationcodes.zip")]
    ArrestLocation,
    #[serde(alias = "bondcodes.zip")]
    #[strum(serialize = "bondcodes.zip")]
    Bond,
    #[serde(alias = "chargephasecodes.zip")]
    #[strum(serialize = "chargephasecodes.zip")]
    ChargePhase,
    #[serde(alias = "citationjurisdictioncodes.zip")]
    #[strum(serialize = "citationjurisdictioncodes.zip")]
    CitationJurisdiction,
    #[serde(alias = "degreecodes.zip")]
    #[strum(serialize = "degreecodes.zip")]
    Degree,
    #[serde(alias = "driverlicensetypecodes.zip")]
    #[strum(serialize = "driverlicensetypecodes.zip")]
    DriverLicenseType,
    #[serde(alias = "ethnicitycodes.zip")]
    #[strum(serialize = "ethnicitycodes.zip")]
    Ethnicity,
    #[serde(alias = "eyecolorcodes.zip")]
    #[strum(serialize = "eyecolorcodes.zip")]
    EyeColor,
    #[serde(alias = "generaloffensecodes.zip")]
    #[strum(serialize = "generaloffensecodes.zip")]
    GeneralOffense,
    #[serde(alias = "haircolorcodes.zip")]
    #[strum(serialize = "haircolorcodes.zip")]
    HairColor,
    #[serde(alias = "lawenforcementunitcodes.zip")]
    #[strum(serialize = "lawenforcementunitcodes.zip")]
    LawEnforcementUnit,
    #[serde(alias = "physicalfeaturecodes.zip")]
    #[strum(serialize = "physicalfeaturecodes.zip")]
    PhysicalFeature,
    #[serde(alias = "racecodes.zip")]
    #[strum(serialize = "racecodes.zip")]
    Race,
    #[serde(alias = "statutecodes.zip")]
    #[strum(serialize = "statutecodes.zip")]
    Statute,
    #[serde(alias = "statutetypecodes.zip")]
    #[strum(serialize = "statutetypecodes.zip")]
    StatuteType,
    #[serde(alias = "vehiclecolorcodes.zip")]
    #[strum(serialize = "vehiclecolorcodes.zip")]
    VehicleColor,
    #[serde(alias = "vehiclemakecodes.zip")]
    #[strum(serialize = "vehiclemakecodes.zip")]
    VehicleMake,
    #[serde(alias = "vehicletypecodes.zip")]
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
            Self::Versions => "codeversions",
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
            Self::Versions => Cow::Borrowed(metadata.version_url()),
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
            Self::Versions
            | Self::Country
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
