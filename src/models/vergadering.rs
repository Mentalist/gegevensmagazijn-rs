use serde::{Deserialize, Serialize};

use super::{TkId, TkObject};

impl TkObject for Vergadering {
    fn entity_set() -> &'static str {
        "Vergadering"
    }
}

/// Vergadering
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/vergadering)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-vergadering.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Vergadering {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "Titel")]
    pub titel: Option<String>,
    #[serde(rename = "Zaal")]
    pub zaal: Option<String>,
    #[serde(rename = "Vergaderjaar")]
    pub vergaderjaar: Option<String>,
    #[serde(rename = "VergaderingNummer")]
    pub vergadering_nummer: Option<i64>,
    #[serde(rename = "Datum")]
    pub datum: Option<String>,
    #[serde(rename = "Aanvangstijd")]
    pub aanvangstijd: Option<String>,
    #[serde(rename = "Sluiting")]
    pub sluiting: Option<String>,
    #[serde(rename = "Kamer")]
    pub kamer: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Verslag")]
    pub verslag: Option<Vec<Verslag>>,
}

impl TkObject for Verslag {
    fn entity_set() -> &'static str {
        "Verslag"
    }
}

/// Verslag
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/verslag)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-verslag.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Verslag {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "ContentType")]
    pub content_type: Option<String>,
    #[serde(rename = "ContentLength")]
    pub content_length: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Vergadering_Id")]
    pub vergadering_id: Option<TkId>,
    #[serde(rename = "Vergadering")]
    pub vergadering: Option<Vergadering>,
}
