use serde::{Deserialize, de::Visitor};

use crate::court_record_service::CaseParty;

pub(super) struct CasePartyRaw<'a> {
    first_name: &'a str,
    middle_name: &'a str,
    last_name: &'a str,
    business_name: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EntityPerson<'a> {
    #[serde(borrow)]
    person_name: PersonName<'a>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PersonName<'a> {
    person_given_name: &'a str,
    person_middle_name: &'a str,
    person_sur_name: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BusinessName<'a> {
    organization_name: &'a str,
}

struct Vis;

impl<'de> Visitor<'de> for Vis {
    type Value = CasePartyRaw<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("ecf:CaseParty/nc:EntityPerson or ecf:CaseParty/nc:EntityOrganization")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut first_name = "";
        let mut middle_name = "";
        let mut last_name = "";
        let mut business_name = "";

        while let Some(key) = map.next_key::<&str>()? {
            if key == "EntityPerson" {
                let person_name: EntityPerson = map.next_value()?;
                first_name = person_name.person_name.person_given_name;
                middle_name = person_name.person_name.person_middle_name;
                last_name = person_name.person_name.person_sur_name;
            } else if key == "EntityOrganization" {
                let business_name_struct: BusinessName = map.next_value()?;
                business_name = business_name_struct.organization_name;
            } else {
                let _: serde::de::IgnoredAny = map.next_value()?;
            }
        }

        Ok(CasePartyRaw {
            first_name,
            middle_name,
            last_name,
            business_name,
        })
    }
}

impl<'de> Deserialize<'de> for CasePartyRaw<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["nc:EntityPerson", "nc:EntityOrganization"];

        deserializer.deserialize_struct("CasePartyRaw", FIELDS, Vis)
    }
}

impl<'a> CasePartyRaw<'a> {
    pub(super) fn into_case_party(self) -> CaseParty<'a> {
        CaseParty {
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            business_name: self.business_name,
        }
    }
}
