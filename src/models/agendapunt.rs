use serde::{Deserialize, Serialize};

use super::{Activiteit, Besluit, Document, TkId, TkObject, Zaak};

impl TkObject for Agendapunt {
    fn entity_set() -> &'static str {
        "Agendapunt"
    }
}

/// Agendapunt
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/agendapunt)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-agendapunt.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Agendapunt {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<String>,
    #[serde(rename = "Onderwerp")]
    pub onderwerp: Option<String>,
    #[serde(rename = "Aanvangstijd")]
    pub aanvangstijd: Option<String>,
    #[serde(rename = "Eindtijd")]
    pub eindtijd: Option<String>,
    #[serde(rename = "Volgorde")]
    pub volgorde: Option<i64>,
    #[serde(rename = "Rubriek")]
    pub rubriek: Option<String>,
    #[serde(rename = "Noot")]
    pub noot: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Activiteit_Id")]
    pub activiteit_id: Option<TkId>,
    #[serde(rename = "Document")]
    pub document: Option<Vec<Document>>,
    #[serde(rename = "Zaak")]
    pub zaak: Option<Vec<Zaak>>,
    #[serde(rename = "Besluit")]
    pub besluit: Option<Vec<Besluit>>,
    #[serde(rename = "Activiteit")]
    pub activiteit: Option<Activiteit>,
}
