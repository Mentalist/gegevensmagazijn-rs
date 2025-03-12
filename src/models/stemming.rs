use serde::{Deserialize, Serialize};

use super::{Besluit, Fractie, Persoon, TkId, TkObject};

impl TkObject for Stemming {
    fn entity_set() -> &'static str {
        "Stemming"
    }
}

/// Stemming
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/stemming)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-stemming.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Stemming {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Besluit_Id")]
    pub besluit_id: Option<TkId>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "FractieGrootte")]
    pub fractie_grootte: Option<i64>,
    #[serde(rename = "ActorNaam")]
    pub actor_naam: Option<String>,
    #[serde(rename = "ActorFractie")]
    pub actor_fractie: Option<String>,
    #[serde(rename = "Vergissing")]
    pub vergissing: Option<bool>,
    #[serde(rename = "SidActorLid")]
    pub sid_actor_lid: Option<String>,
    #[serde(rename = "SidActorFractie")]
    pub sid_actor_fractie: Option<String>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Fractie_Id")]
    pub fractie_id: Option<TkId>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Besluit")]
    pub besluit: Option<Besluit>,
    #[serde(rename = "Fractie")]
    pub fractie: Option<Fractie>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}
