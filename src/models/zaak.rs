use serde::{Deserialize, Serialize};

use super::{
    Activiteit, Agendapunt, Besluit, Commissie, Document, Fractie,
    Kamerstukdossier, Persoon, TkId, TkObject,
};

impl TkObject for Zaak {
    fn entity_set() -> &'static str {
        "Zaak"
    }
}

/// Zaak
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/zaak)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-zaak.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Zaak {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<String>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "Titel")]
    pub titel: Option<String>,
    #[serde(rename = "Citeertitel")]
    pub citeertitel: Option<String>,
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Onderwerp")]
    pub onderwerp: Option<String>,
    #[serde(rename = "GestartOp")]
    pub gestart_op: Option<String>,
    #[serde(rename = "Organisatie")]
    pub organisatie: Option<String>,
    #[serde(rename = "Grondslagvoorhang")]
    pub grondslagvoorhang: Option<String>,
    #[serde(rename = "Termijn")]
    pub termijn: Option<String>,
    #[serde(rename = "Vergaderjaar")]
    pub vergaderjaar: Option<String>,
    #[serde(rename = "Volgnummer")]
    pub volgnummer: Option<i64>,
    #[serde(rename = "HuidigeBehandelstatus")]
    pub huidige_behandelstatus: Option<String>,
    #[serde(rename = "Afgedaan")]
    pub afgedaan: Option<bool>,
    #[serde(rename = "GrootProject")]
    pub groot_project: Option<bool>,
    #[serde(rename = "Kabinetsappreciatie")]
    pub kabinetsappreciatie: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "ZaakActor")]
    pub zaak_actor: Option<Vec<ZaakActor>>,
    #[serde(rename = "Activiteit")]
    pub activiteit: Option<Vec<Activiteit>>,
    #[serde(rename = "Agendapunt")]
    pub agendapunt: Option<Vec<Agendapunt>>,
    #[serde(rename = "Besluit")]
    pub besluit: Option<Vec<Besluit>>,
    #[serde(rename = "Document")]
    pub document: Option<Vec<Document>>,
    #[serde(rename = "Kamerstukdossier")]
    pub kamerstukdossier: Option<Vec<Kamerstukdossier>>,
}

impl TkObject for ZaakActor {
    fn entity_set() -> &'static str {
        "ZaakActor"
    }
}

/// ZaakActor
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/zaakactor)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-zaakactor.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ZaakActor {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Zaak_Id")]
    pub zaak_id: Option<TkId>,
    #[serde(rename = "ActorNaam")]
    pub actor_naam: Option<String>,
    #[serde(rename = "ActorFractie")]
    pub actor_fractie: Option<String>,
    #[serde(rename = "ActorAfkorting")]
    pub actor_afkorting: Option<String>,
    #[serde(rename = "Functie")]
    pub functie: Option<String>,
    #[serde(rename = "Relatie")]
    pub relatie: Option<String>,
    #[serde(rename = "SidActor")]
    pub sid_actor: Option<String>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Fractie_Id")]
    pub fractie_id: Option<TkId>,
    #[serde(rename = "Commissie_Id")]
    pub commissie_id: Option<TkId>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Commissie")]
    pub commissie: Option<Commissie>,
    #[serde(rename = "Fractie")]
    pub fractie: Option<Fractie>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
    #[serde(rename = "Zaak")]
    pub zaak: Option<Zaak>,
}
