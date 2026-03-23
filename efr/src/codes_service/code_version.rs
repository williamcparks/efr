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

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString, Deserialize, Serialize)]
pub enum CodeVersionFile {
    // ── System-wide ──────────────────────────────────────────────────────────
    #[strum(serialize = "countrycodes.zip")]
    CountryCodes,
    #[strum(serialize = "datafieldconfigcodes.zip")]
    DataFieldConfigCodes,
    #[strum(serialize = "errorcodes.zip")]
    ErrorCodes,
    #[strum(serialize = "filingstatuscodes.zip")]
    FilingStatusCodes,
    #[strum(serialize = "languagecodes.zip")]
    LanguageCodes,
    #[strum(serialize = "locations.zip")]
    Locations,
    #[strum(serialize = "statecodes.zip")]
    StateCodes,

    // ── Court-specific ───────────────────────────────────────────────────────
    #[strum(serialize = "answercodes.zip")]
    AnswerCodes,
    #[strum(serialize = "appellatelowercourts.zip")]
    AppellateLowerCourts,
    #[strum(serialize = "casecategorycodes.zip")]
    CaseCategoryCodes,
    #[strum(serialize = "casesubtypecodes.zip")]
    CaseSubTypeCodes,
    #[strum(serialize = "casetypecodes.zip")]
    CaseTypeCodes,
    #[strum(serialize = "crossreferencecodes.zip")]
    CrossReferenceCodes,
    #[strum(serialize = "damageamountcodes.zip")]
    DamageAmountCodes,
    #[strum(serialize = "disclaimerrequirementcodes.zip")]
    DisclaimerRequirementCodes,
    #[strum(serialize = "documenttypecodes.zip")]
    DocumentTypeCodes,
    #[strum(serialize = "filertypecodes.zip")]
    FilerTypeCodes,
    #[strum(serialize = "filetypecodes.zip")]
    FileTypeCodes,
    #[strum(serialize = "filingcodes.zip")]
    FilingCodes,
    #[strum(serialize = "filingcomponentcodes.zip")]
    FilingComponentCodes,
    #[strum(serialize = "hearinglocationcodes.zip")]
    HearingLocationCodes,
    #[strum(serialize = "judicialofficercodes.zip")]
    JudicialOfficerCodes,
    #[strum(serialize = "motiontypecodes.zip")]
    MotionTypeCodes,
    #[strum(serialize = "namesuffixcodes.zip")]
    NameSuffixCodes,
    #[strum(serialize = "notificationagencycodes.zip")]
    NotificationAgencyCodes,
    #[strum(serialize = "optionalservicescodes.zip")]
    OptionalServicesCodes,
    #[strum(serialize = "partytypecodes.zip")]
    PartyTypeCodes,
    #[strum(serialize = "procedureremedycodes.zip")]
    ProcedureRemedyCodes,
    #[strum(serialize = "questioncodes.zip")]
    QuestionCodes,
    #[strum(serialize = "refundreasoncodes.zip")]
    RefundReasonCodes,
    #[strum(serialize = "repcapcodes.zip")]
    RepCapCodes,
    #[strum(serialize = "serviceprovidercodes.zip")]
    ServiceProviderCodes,
    #[strum(serialize = "servicetypecodes.zip")]
    ServiceTypeCodes,

    // ── Criminal initiation ──────────────────────────────────────────────────
    #[strum(serialize = "arrestlocationcodes.zip")]
    ArrestLocationCodes,
    #[strum(serialize = "bondcodes.zip")]
    BondCodes,
    #[strum(serialize = "chargephasecodes.zip")]
    ChargePhaseCodes,
    #[strum(serialize = "citationjurisdictioncodes.zip")]
    CitationJurisdictionCodes,
    #[strum(serialize = "degreecodes.zip")]
    DegreeCodes,
    #[strum(serialize = "driverlicensetypecodes.zip")]
    DriverLicenseTypeCodes,
    #[strum(serialize = "ethnicitycodes.zip")]
    EthnicityCodes,
    #[strum(serialize = "eyecolorcodes.zip")]
    EyeColorCodes,
    #[strum(serialize = "generaloffensecodes.zip")]
    GeneralOffenseCodes,
    #[strum(serialize = "haircolorcodes.zip")]
    HairColorCodes,
    #[strum(serialize = "lawenforcementunitcodes.zip")]
    LawEnforcementUnitCodes,
    #[strum(serialize = "physicalfeaturecodes.zip")]
    PhysicalFeatureCodes,
    #[strum(serialize = "racecodes.zip")]
    RaceCodes,
    #[strum(serialize = "statutecodes.zip")]
    StatuteCodes,
    #[strum(serialize = "statutetypecodes.zip")]
    StatuteTypeCodes,
    #[strum(serialize = "vehiclecolorcodes.zip")]
    VehicleColorCodes,
    #[strum(serialize = "vehiclemakecodes.zip")]
    VehicleMakeCodes,
    #[strum(serialize = "vehicletypecodes.zip")]
    VehicleTypeCodes,
}

#[derive(Clone, Debug, Deserialize, Serialize, CodeList)]
#[codelist("Code Version")]
struct RawCodeVersion<'a> {
    location: Cow<'a, str>,
    codelist: Cow<'a, str>,
    version: Cow<'a, str>,
}
