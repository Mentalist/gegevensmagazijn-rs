use serde::{Deserialize, Serialize};

use super::{Document, TkId, TkObject, Zaak};

impl TkObject for Kamerstukdossier {
    fn entity_set() -> &'static str {
        "Kamerstukdossier"
    }
}

/// Kamerstukdossier
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/kamerstukdossier)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-kamerstukdossier.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Kamerstukdossier {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Titel")]
    pub titel: Option<String>,
    #[serde(rename = "Citeertitel")]
    pub citeertitel: Option<String>,
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<i64>,
    #[serde(rename = "Toevoeging")]
    pub toevoeging: Option<String>,
    #[serde(rename = "HoogsteVolgnummer")]
    pub hoogste_volgnummer: Option<i64>,
    #[serde(rename = "Afgesloten")]
    pub afgesloten: Option<bool>,
    #[serde(rename = "Kamer")]
    pub kamer: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Document")]
    pub document: Option<Vec<Document>>,
    #[serde(rename = "Zaak")]
    pub zaak: Option<Vec<Zaak>>,
}
