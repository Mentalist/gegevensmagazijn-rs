use serde::{Deserialize, Serialize};

use super::{Agendapunt, Stemming, TkId, TkObject, Zaak};

impl TkObject for Besluit {
    fn entity_set() -> &'static str {
        "Besluit"
    }
}

/// Besluit
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/besluit)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-besluit.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Besluit {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Agendapunt_Id")]
    pub agendapunt_id: Option<TkId>,
    #[serde(rename = "StemmingsSoort")]
    pub stemmings_soort: Option<String>,
    #[serde(rename = "BesluitSoort")]
    pub besluit_soort: Option<String>,
    #[serde(rename = "BesluitTekst")]
    pub besluit_tekst: Option<String>,
    #[serde(rename = "Opmerking")]
    pub opmerking: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "AgendapuntZaakBesluitVolgorde")]
    pub agendapunt_zaak_besluit_volgorde: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Stemming")]
    pub stemming: Option<Vec<Stemming>>,
    #[serde(rename = "Agendapunt")]
    pub agendapunt: Option<Agendapunt>,
    #[serde(rename = "Zaak")]
    pub zaak: Option<Vec<Zaak>>,
}
