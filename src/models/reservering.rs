use serde::{Deserialize, Serialize};

use super::{Activiteit, TkId, TkObject, Zaal};

impl TkObject for Reservering {
    fn entity_set() -> &'static str {
        "Reservering"
    }
}

/// Reservering
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/reservering)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-reservering.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Reservering {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<String>,
    #[serde(rename = "StatusCode")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusNaam")]
    pub status_naam: Option<String>,
    #[serde(rename = "ActiviteitNummer")]
    pub activiteit_nummer: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Activiteit")]
    pub activiteit: Option<Vec<Activiteit>>,
    #[serde(rename = "Zaal")]
    pub zaal: Option<Vec<Zaal>>,
}
