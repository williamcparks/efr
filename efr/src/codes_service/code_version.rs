use std::{borrow::Cow, str::FromStr};

use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::codes_service::{EfrCodesError, utils::CodeRow};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeVersion<'a> {
    pub location: Cow<'a, str>,
    pub codelist: CodeVersionFile,
    pub version: Cow<'a, str>,
}

impl<'a> CodeRow<'a> for CodeVersion<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeVersion::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        let codelist = match CodeVersionFile::from_str(raw.codelist.as_ref()) {
            Ok(ok) => ok,
            Err(err) => {
                return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                    "Failed To Parse `codelist`: `{err}` Got: `{}`",
                    raw.codelist
                ))));
            }
        };

        Ok(Some(Self {
            location: raw.location,
            codelist,
            version: raw.version,
        }))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, EnumString, Deserialize, Serialize)]
pub enum CodeVersionFile {
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

impl CodeVersionFile {
    pub fn to_static_str(&self) -> &'static str {
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
}

#[derive(Clone, Debug, Deserialize, Serialize, CodeList)]
#[codelist("Code Version")]
struct RawCodeVersion<'a> {
    location: Cow<'a, str>,
    codelist: Cow<'a, str>,
    version: Cow<'a, str>,
}
