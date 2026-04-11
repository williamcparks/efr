use bitflags::bitflags;
use efr_macros::CodeList;
use quick_xml::NsReader;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::codes_service::EfrCodesError;

use super::utils::CodeRow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeLocation<'a> {
    pub code: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub odysseynodeid: i64,
    pub cmsid: i64,
    pub parentnodeid: Cow<'a, str>,
    pub allowablecardtypes: CodeLocationAllowableCardTypes,
    pub protectedcasetypes: Cow<'a, str>,
    pub redactionurl: Cow<'a, str>,
    pub redactionviewerurl: Cow<'a, str>,
    pub redactionapiversion: Cow<'a, str>,
    pub redactiondocumenttype: Cow<'a, str>,
    pub defaultdocumentdescription: Cow<'a, str>,
    pub protectedcasereplacementstring: Cow<'a, str>,
    pub redactionfee: f64,
    pub redactiontargetconfiguration: CodeLocationRedactionTargetConfig,
    pub partialwaiverdurationindays: i64,
    pub partialwaivercourtpaymentsystemurl: Cow<'a, str>,
    pub partialwaiveravailablewaiverpercentages: CodeLocationPartialWaiver,
    pub eserviceconsenttextblurbmain: Cow<'a, str>,
    pub eserviceconsenttextblurbsecondary: Cow<'a, str>,
    pub eserviceconsenttextblurbsecondaryafterconsentyes: Cow<'a, str>,
    pub eserviceconsenttextconsentyes: Cow<'a, str>,
    pub eserviceconsenttextconsentyeshelp: Cow<'a, str>,
    pub eserviceconsenttextconsentyeswithadd: Cow<'a, str>,
    pub eserviceconsenttextconsentyeswithaddhelp: Cow<'a, str>,
    pub eserviceconsenttextconsentno: Cow<'a, str>,
    pub eserviceconsenttextconsentnohelp: Cow<'a, str>,
    pub promptforconfidentialdocuments: Cow<'a, str>,
    pub defaultdocumentsecurity: Cow<'a, str>,
    pub cmsservicecontactsupdatesfirmid: Cow<'a, str>,
    pub cmsservicecontactsupdatesbehavior: Cow<'a, str>,
    pub chargenumberconfigurationregularexpression: Cow<'a, str>,
    pub chargenumberconfigurationghosttext: Cow<'a, str>,
    pub chargenumberconfigurationvalidationmessage: Cow<'a, str>,
    pub flags: CodeLocationFlags,
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
    pub struct CodeLocationFlags: u64 {
        const INITIAL                                    = 1 << 0;
        const SUBSEQUENT                                 = 1 << 1;
        const DISALLOW_COPYING_ENVELOPE_MULTIPLE_TIMES   = 1 << 2;
        const ALLOW_FILING_INTO_NON_INDEXED_CASE         = 1 << 3;
        const SEND_SERVICE_BEFORE_REVIEW                 = 1 << 4;
        const IS_COUNTY                                  = 1 << 5;
        const RESTRICT_BANK_ACCOUNT_PAYMENT              = 1 << 6;
        const ALLOW_MULTIPLE_ATTORNEYS                   = 1 << 7;
        const SEND_SERVICE_CONTACT_REMOVED_NOTIFICATIONS = 1 << 8;
        const ALLOW_MAX_FEE_AMOUNT                       = 1 << 9;
        const TRANSFER_WAIVED_FEES_TO_CMS                = 1 << 10;
        const SKIP_PRE_AUTH                              = 1 << 11;
        const ALLOW_HEARING                              = 1 << 12;
        const ALLOW_RETURN_DATE                          = 1 << 13;
        const SHOW_DAMAGE_AMOUNT                         = 1 << 14;
        const HAS_CONDITIONAL_SERVICE_TYPES              = 1 << 15;
        const HAS_PROTECTED_CASE_TYPES                   = 1 << 16;
        const ALLOW_ZERO_FEES_WITHOUT_FILING_PARTY       = 1 << 17;
        const ALLOW_SERVICE_ON_INITIAL                   = 1 << 18;
        const ALLOW_ADD_SERVICE_CONTACTS_ON_INITIAL      = 1 << 19;
        const ALLOW_REDACTION                            = 1 << 20;
        const ENFORCE_REDACTION                          = 1 << 21;
        const ALLOW_WAIVER_ON_MAIL                       = 1 << 22;
        const SHOW_RETURN_ON_REJECT                      = 1 << 23;
        const ALLOW_CHARGE_UPDATE                        = 1 << 24;
        const ALLOW_PARTY_ID                             = 1 << 25;
        const ALLOW_WAIVER_ON_REDACTION                  = 1 << 26;
        const DISALLOW_ELECTRONIC_SERVICE_ON_NEW_CONTACTS = 1 << 27;
        const ALLOW_INDIVIDUAL_REGISTRATION              = 1 << 28;
        const BULK_FILING_FEE_ASSESSOR_CONFIGURATION     = 1 << 29;
        const ENVELOPE_LEVEL_COMMENT_CONFIGURATION       = 1 << 30;
        const AUTO_ASSIGN_SRL_SERVICE_CONTACT            = 1 << 31;
        const AUTO_ASSIGN_ATTORNEY_SERVICE_CONTACT       = 1 << 32;
        const ALLOW_REP_CAP                              = 1 << 33;
        const ESERVICE_CONSENT_ENABLED                   = 1 << 34;
        const PROMPT_FOR_CONFIDENTIAL_DOCUMENTS_ENABLED  = 1 << 35;
        const DEFAULT_DOCUMENT_SECURITY_ENABLED          = 1 << 36;
        const CMS_SERVICE_CONTACTS_UPDATES_ENABLED       = 1 << 37;
        const SUBSEQUENT_ACTIONS_ENABLED                 = 1 << 38;
        const HEARING_SCHEDULING_ENABLED                 = 1 << 39;
        const SERVICE_OF_PROCESS_ENABLED                 = 1 << 40;
        const CHARGE_NUMBER_CONFIGURATION_ENABLED        = 1 << 41;
        const CHARGE_NUMBER_CONFIGURATION_REQUIRED       = 1 << 42;
        const CHARGE_NUMBER_CONFIGURATION_AVAILABLE      = 1 << 43;
        const CHARGE_NUMBER_CONFIGURATION_USE_DEFAULT_SEQUENCE = 1 << 44;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
    pub struct CodeLocationAllowableCardTypes: u8 {
        const AMEX = 1 << 0;
        const CHECKING = 1 << 1;
        const DISCOVER = 1 << 2;
        const MASTERCARD = 1 << 3;
        const VISA = 1 << 4;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
    pub struct CodeLocationRedactionTargetConfig: u8 {
        const ACCOUNT_NUMBER = 1 << 0;
        const CREDIT_CARD = 1 << 1;
        const DRIVERS_LICENSE = 1 << 2;
        const GOVERNMENT_ID = 1 << 3;
        const PASSPORT = 1 << 4;
        const SOCIAL_SECURITY_NUMBER = 1 << 5;
        const TAX_DOCUMENT = 1 << 6;
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CodeLocationPartialWaiver([u8; 8]);

impl<'a> CodeRow<'a> for CodeLocation<'a> {
    fn code_row(ns_reader: &mut NsReader<&'a [u8]>) -> Result<Option<Self>, EfrCodesError> {
        let raw = match RawCodeLocation::code_row(ns_reader) {
            Ok(Some(some)) => some,
            Ok(None) => return Ok(None),
            Err(err) => return Err(err),
        };

        let odysseynodeid = match raw.odysseynodeid.parse() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                    "Failed To Parse `odysseynodeid`: `{err}` Got: `{}`",
                    raw.odysseynodeid
                ))));
            }
        };
        let cmsid = match raw.cmsid.is_empty() {
            true => 0,
            false => match raw.cmsid.parse() {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Failed To Parse `cmsid`: `{err}` Got: `{}`",
                        raw.cmsid
                    ))));
                }
            },
        };

        let allowablecardtypes =
            CodeLocationAllowableCardTypes::try_new(raw.allowablecardtypes.as_ref())?;

        let redactionfee = match raw.redactionfee.parse() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                    "Failed To Parse `redactionfee`: `{err}` Got: `{}`",
                    raw.redactionfee
                ))));
            }
        };

        let redactiontargetconfiguration =
            CodeLocationRedactionTargetConfig::try_new(raw.redactiontargetconfiguration.as_ref())?;

        let partialwaiverdurationindays = match raw.partialwaiverdurationindays.parse() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                    "Failed To Parse `partialwaiverdurationindays`: `{err}` Got: `{}`",
                    raw.partialwaiverdurationindays
                ))));
            }
        };

        let partialwaiveravailablewaiverpercentages = CodeLocationPartialWaiver::try_new(
            raw.partialwaiveravailablewaiverpercentages.as_ref(),
        )?;

        let mut flags = CodeLocationFlags::empty();
        if raw.initial.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::INITIAL;
        }
        if raw.subsequent.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::SUBSEQUENT;
        }
        if raw
            .disallowcopyingenvelopemultipletimes
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::DISALLOW_COPYING_ENVELOPE_MULTIPLE_TIMES;
        }
        if raw
            .allowfilingintononindexedcase
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::ALLOW_FILING_INTO_NON_INDEXED_CASE;
        }
        if raw.sendservicebeforereview.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::SEND_SERVICE_BEFORE_REVIEW;
        }
        if raw.iscounty.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::IS_COUNTY;
        }
        if raw.restrictbankaccountpayment.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::RESTRICT_BANK_ACCOUNT_PAYMENT;
        }
        if raw.allowmultipleattorneys.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_MULTIPLE_ATTORNEYS;
        }
        if raw
            .sendservicecontactremovednotifications
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::SEND_SERVICE_CONTACT_REMOVED_NOTIFICATIONS;
        }
        if raw.allowmaxfeeamount.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_MAX_FEE_AMOUNT;
        }
        if raw.transferwaivedfeestocms.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::TRANSFER_WAIVED_FEES_TO_CMS;
        }
        if raw.skippreauth.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::SKIP_PRE_AUTH;
        }
        if raw.allowhearing.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_HEARING;
        }
        if raw.allowreturndate.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_RETURN_DATE;
        }
        if raw.showdamageamount.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::SHOW_DAMAGE_AMOUNT;
        }
        if raw.hasconditionalservicetypes.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::HAS_CONDITIONAL_SERVICE_TYPES;
        }
        if raw.hasprotectedcasetypes.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::HAS_PROTECTED_CASE_TYPES;
        }
        if raw
            .allowzerofeeswithoutfilingparty
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::ALLOW_ZERO_FEES_WITHOUT_FILING_PARTY;
        }
        if raw.allowserviceoninitial.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_SERVICE_ON_INITIAL;
        }
        if raw
            .allowaddservicecontactsoninitial
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::ALLOW_ADD_SERVICE_CONTACTS_ON_INITIAL;
        }
        if raw.allowredaction.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_REDACTION;
        }
        if raw.enforceredaction.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ENFORCE_REDACTION;
        }
        if raw.allowwaiveronmail.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_WAIVER_ON_MAIL;
        }
        if raw.showreturnonreject.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::SHOW_RETURN_ON_REJECT;
        }
        if raw.allowchargeupdate.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_CHARGE_UPDATE;
        }
        if raw.allowpartyid.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_PARTY_ID;
        }
        if raw.allowwaiveronredaction.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_WAIVER_ON_REDACTION;
        }
        if raw
            .disallowelectronicserviceonnewcontacts
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::DISALLOW_ELECTRONIC_SERVICE_ON_NEW_CONTACTS;
        }
        if raw.allowindividualregistration.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_INDIVIDUAL_REGISTRATION;
        }
        if raw
            .bulkfilingfeeassessorconfiguration
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::BULK_FILING_FEE_ASSESSOR_CONFIGURATION;
        }
        if raw
            .envelopelevelcommentconfiguration
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::ENVELOPE_LEVEL_COMMENT_CONFIGURATION;
        }
        if raw.autoassignsrlservicecontact.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::AUTO_ASSIGN_SRL_SERVICE_CONTACT;
        }
        if raw
            .autoassignattorneyservicecontact
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::AUTO_ASSIGN_ATTORNEY_SERVICE_CONTACT;
        }
        if raw.allowrepcap.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ALLOW_REP_CAP;
        }
        if raw.eserviceconsentenabled.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::ESERVICE_CONSENT_ENABLED;
        }
        if raw
            .promptforconfidentialdocumentsenabled
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::PROMPT_FOR_CONFIDENTIAL_DOCUMENTS_ENABLED;
        }
        if raw
            .defaultdocumentsecurityenabled
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::DEFAULT_DOCUMENT_SECURITY_ENABLED;
        }
        if raw
            .cmsservicecontactsupdatesenabled
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::CMS_SERVICE_CONTACTS_UPDATES_ENABLED;
        }
        if raw.subsequentactionsenabled.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::SUBSEQUENT_ACTIONS_ENABLED;
        }
        if raw.hearingschedulingenabled.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::HEARING_SCHEDULING_ENABLED;
        }
        if raw.serviceofprocessenabled.eq_ignore_ascii_case("true") {
            flags |= CodeLocationFlags::SERVICE_OF_PROCESS_ENABLED;
        }
        if raw
            .chargenumberconfigurationenabled
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::CHARGE_NUMBER_CONFIGURATION_ENABLED;
        }
        if raw
            .chargenumberconfigurationrequired
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::CHARGE_NUMBER_CONFIGURATION_REQUIRED;
        }
        if raw
            .chargenumberconfigurationavailable
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::CHARGE_NUMBER_CONFIGURATION_AVAILABLE;
        }
        if raw
            .chargenumberconfigurationusedefaultsequence
            .eq_ignore_ascii_case("true")
        {
            flags |= CodeLocationFlags::CHARGE_NUMBER_CONFIGURATION_USE_DEFAULT_SEQUENCE;
        }

        Ok(Some(Self {
            code: raw.code,
            name: raw.name,
            odysseynodeid,
            cmsid,
            parentnodeid: raw.parentnodeid,
            allowablecardtypes,
            protectedcasetypes: raw.protectedcasetypes,
            redactionurl: raw.redactionurl,
            redactionviewerurl: raw.redactionviewerurl,
            redactionapiversion: raw.redactionapiversion,
            redactiondocumenttype: raw.redactiondocumenttype,
            defaultdocumentdescription: raw.defaultdocumentdescription,
            protectedcasereplacementstring: raw.protectedcasereplacementstring,
            redactionfee,
            redactiontargetconfiguration,
            partialwaiverdurationindays,
            partialwaivercourtpaymentsystemurl: raw.partialwaivercourtpaymentsystemurl,
            partialwaiveravailablewaiverpercentages,
            eserviceconsenttextblurbmain: raw.eserviceconsenttextblurbmain,
            eserviceconsenttextblurbsecondary: raw.eserviceconsenttextblurbsecondary,
            eserviceconsenttextblurbsecondaryafterconsentyes: raw
                .eserviceconsenttextblurbsecondaryafterconsentyes,
            eserviceconsenttextconsentyes: raw.eserviceconsenttextconsentyes,
            eserviceconsenttextconsentyeshelp: raw.eserviceconsenttextconsentyeshelp,
            eserviceconsenttextconsentyeswithadd: raw.eserviceconsenttextconsentyeswithadd,
            eserviceconsenttextconsentyeswithaddhelp: raw.eserviceconsenttextconsentyeswithaddhelp,
            eserviceconsenttextconsentno: raw.eserviceconsenttextconsentno,
            eserviceconsenttextconsentnohelp: raw.eserviceconsenttextconsentnohelp,
            promptforconfidentialdocuments: raw.promptforconfidentialdocuments,
            defaultdocumentsecurity: raw.defaultdocumentsecurity,
            cmsservicecontactsupdatesfirmid: raw.cmsservicecontactsupdatesfirmid,
            cmsservicecontactsupdatesbehavior: raw.cmsservicecontactsupdatesbehavior,
            chargenumberconfigurationregularexpression: raw
                .chargenumberconfigurationregularexpression,
            chargenumberconfigurationghosttext: raw.chargenumberconfigurationghosttext,
            chargenumberconfigurationvalidationmessage: raw
                .chargenumberconfigurationvalidationmessage,
            flags,
        }))
    }
}

#[derive(CodeList)]
#[codelist("Code Location")]
struct RawCodeLocation<'a> {
    code: Cow<'a, str>,
    #[codelist(optional)]
    name: Cow<'a, str>,
    #[codelist(optional)]
    initial: Cow<'a, str>,
    #[codelist(optional)]
    subsequent: Cow<'a, str>,
    #[codelist(optional)]
    disallowcopyingenvelopemultipletimes: Cow<'a, str>,
    #[codelist(optional)]
    allowfilingintononindexedcase: Cow<'a, str>,
    #[codelist(optional)]
    allowablecardtypes: Cow<'a, str>,
    #[codelist(optional)]
    odysseynodeid: Cow<'a, str>,
    #[codelist(optional)]
    cmsid: Cow<'a, str>,
    #[codelist(optional)]
    sendservicebeforereview: Cow<'a, str>,
    #[codelist(optional)]
    parentnodeid: Cow<'a, str>,
    #[codelist(optional)]
    iscounty: Cow<'a, str>,
    #[codelist(optional)]
    restrictbankaccountpayment: Cow<'a, str>,
    #[codelist(optional)]
    allowmultipleattorneys: Cow<'a, str>,
    #[codelist(optional)]
    sendservicecontactremovednotifications: Cow<'a, str>,
    #[codelist(optional)]
    allowmaxfeeamount: Cow<'a, str>,
    #[codelist(optional)]
    transferwaivedfeestocms: Cow<'a, str>,
    #[codelist(optional)]
    skippreauth: Cow<'a, str>,
    #[codelist(optional)]
    allowhearing: Cow<'a, str>,
    #[codelist(optional)]
    allowreturndate: Cow<'a, str>,
    #[codelist(optional)]
    showdamageamount: Cow<'a, str>,
    #[codelist(optional)]
    hasconditionalservicetypes: Cow<'a, str>,
    #[codelist(optional)]
    hasprotectedcasetypes: Cow<'a, str>,
    #[codelist(optional)]
    protectedcasetypes: Cow<'a, str>,
    #[codelist(optional)]
    allowzerofeeswithoutfilingparty: Cow<'a, str>,
    #[codelist(optional)]
    allowserviceoninitial: Cow<'a, str>,
    #[codelist(optional)]
    allowaddservicecontactsoninitial: Cow<'a, str>,
    #[codelist(optional)]
    allowredaction: Cow<'a, str>,
    #[codelist(optional)]
    redactionurl: Cow<'a, str>,
    #[codelist(optional)]
    redactionviewerurl: Cow<'a, str>,
    #[codelist(optional)]
    redactionapiversion: Cow<'a, str>,
    #[codelist(optional)]
    enforceredaction: Cow<'a, str>,
    #[codelist(optional)]
    redactiondocumenttype: Cow<'a, str>,
    #[codelist(optional)]
    defaultdocumentdescription: Cow<'a, str>,
    #[codelist(optional)]
    allowwaiveronmail: Cow<'a, str>,
    #[codelist(optional)]
    showreturnonreject: Cow<'a, str>,
    #[codelist(optional)]
    protectedcasereplacementstring: Cow<'a, str>,
    #[codelist(optional)]
    allowchargeupdate: Cow<'a, str>,
    #[codelist(optional)]
    allowpartyid: Cow<'a, str>,
    #[codelist(optional)]
    redactionfee: Cow<'a, str>,
    #[codelist(optional)]
    allowwaiveronredaction: Cow<'a, str>,
    #[codelist(optional)]
    disallowelectronicserviceonnewcontacts: Cow<'a, str>,
    #[codelist(optional)]
    allowindividualregistration: Cow<'a, str>,
    #[codelist(optional)]
    redactiontargetconfiguration: Cow<'a, str>,
    #[codelist(optional)]
    bulkfilingfeeassessorconfiguration: Cow<'a, str>,
    #[codelist(optional)]
    envelopelevelcommentconfiguration: Cow<'a, str>,
    #[codelist(optional)]
    autoassignsrlservicecontact: Cow<'a, str>,
    #[codelist(optional)]
    autoassignattorneyservicecontact: Cow<'a, str>,
    #[codelist(optional)]
    partialwaiverdurationindays: Cow<'a, str>,
    #[codelist(optional)]
    partialwaivercourtpaymentsystemurl: Cow<'a, str>,
    #[codelist(optional)]
    partialwaiveravailablewaiverpercentages: Cow<'a, str>,
    #[codelist(optional)]
    allowrepcap: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsentenabled: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextblurbmain: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextblurbsecondary: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextblurbsecondaryafterconsentyes: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextconsentyes: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextconsentyeshelp: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextconsentyeswithadd: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextconsentyeswithaddhelp: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextconsentno: Cow<'a, str>,
    #[codelist(optional)]
    eserviceconsenttextconsentnohelp: Cow<'a, str>,
    #[codelist(optional)]
    promptforconfidentialdocumentsenabled: Cow<'a, str>,
    #[codelist(optional)]
    promptforconfidentialdocuments: Cow<'a, str>,
    #[codelist(optional)]
    defaultdocumentsecurityenabled: Cow<'a, str>,
    #[codelist(optional)]
    defaultdocumentsecurity: Cow<'a, str>,
    #[codelist(optional)]
    cmsservicecontactsupdatesenabled: Cow<'a, str>,
    #[codelist(optional)]
    cmsservicecontactsupdatesfirmid: Cow<'a, str>,
    #[codelist(optional)]
    cmsservicecontactsupdatesbehavior: Cow<'a, str>,
    #[codelist(optional)]
    subsequentactionsenabled: Cow<'a, str>,
    #[codelist(optional)]
    hearingschedulingenabled: Cow<'a, str>,
    #[codelist(optional)]
    serviceofprocessenabled: Cow<'a, str>,
    #[codelist(optional)]
    chargenumberconfigurationenabled: Cow<'a, str>,
    #[codelist(optional)]
    chargenumberconfigurationrequired: Cow<'a, str>,
    #[codelist(optional)]
    chargenumberconfigurationavailable: Cow<'a, str>,
    #[codelist(optional)]
    chargenumberconfigurationusedefaultsequence: Cow<'a, str>,
    #[codelist(optional)]
    chargenumberconfigurationregularexpression: Cow<'a, str>,
    #[codelist(optional)]
    chargenumberconfigurationghosttext: Cow<'a, str>,
    #[codelist(optional)]
    chargenumberconfigurationvalidationmessage: Cow<'a, str>,
}

impl CodeLocationAllowableCardTypes {
    fn try_new(list: &str) -> Result<Self, EfrCodesError> {
        let mut res = Self::empty();

        for el in list.split(',') {
            let card_type = match el {
                "AMEX" => Self::AMEX,
                "CHECKING" => Self::CHECKING,
                "DISCOVER" => Self::DISCOVER,
                "MASTERCARD" => Self::MASTERCARD,
                "VISA" => Self::VISA,
                other => {
                    return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Unknown Card Type: `{other}`"
                    ))));
                }
            };

            res |= card_type;
        }

        Ok(res)
    }
}

impl CodeLocationRedactionTargetConfig {
    fn try_new(list: &str) -> Result<Self, EfrCodesError> {
        let mut res = Self::empty();

        for el in list.split(',') {
            let card_type = match el {
                "AccountNumber" => Self::ACCOUNT_NUMBER,
                "CreditCard" => Self::CREDIT_CARD,
                "DriversLicense" => Self::DRIVERS_LICENSE,
                "GovernmentId" => Self::GOVERNMENT_ID,
                "Passport" => Self::PASSPORT,
                "SocialSecurityNumber" => Self::SOCIAL_SECURITY_NUMBER,
                "TaxDocument" => Self::TAX_DOCUMENT,
                other => {
                    return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Unknown Redaction Target: `{other}`"
                    ))));
                }
            };

            res |= card_type;
        }

        Ok(res)
    }
}

impl CodeLocationPartialWaiver {
    fn try_new(raw: &str) -> Result<Self, EfrCodesError> {
        let mut res = [0; 8];

        for (idx, el) in raw.split(',').enumerate() {
            let parsed: u8 = match el.parse() {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Failed To Parse `partialwaiveravailablewaiverpercentages`: `{err}` Got: `{el}` In `{raw}`"
                    ))));
                }
            };

            let ptr = match res.get_mut(idx) {
                Some(some) => some,
                None => {
                    return Err(EfrCodesError::Xml(quick_xml::DeError::Custom(format!(
                        "Failed To Parse `partialwaiveravailablewaiverpercentages`: `There were more than 8 percentages` In `{raw}`"
                    ))));
                }
            };

            *ptr = parsed;
        }

        Ok(Self(res))
    }
}

impl Serialize for CodeLocationPartialWaiver {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bits = u64::from_le_bytes(self.0);
        serializer.serialize_u64(bits)
    }
}

impl<'de> Deserialize<'de> for CodeLocationPartialWaiver {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bits = u64::deserialize(deserializer)?;
        Ok(Self(bits.to_le_bytes()))
    }
}
