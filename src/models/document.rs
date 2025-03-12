use serde::{Deserialize, Serialize};

use super::{
    Activiteit, Agendapunt, Fractie, Kamerstukdossier, Persoon, TkId, TkObject,
    TkResource, Zaak,
};

impl TkResource for Document {}

impl TkObject for Document {
    fn entity_set() -> &'static str {
        "Document"
    }
}

/// Document
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/document)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-document.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "DocumentNummer")]
    pub document_nummer: Option<String>,
    #[serde(rename = "Titel")]
    pub titel: Option<String>,
    #[serde(rename = "Onderwerp")]
    pub onderwerp: Option<String>,
    #[serde(rename = "Datum")]
    pub datum: Option<String>,
    #[serde(rename = "Vergaderjaar")]
    pub vergaderjaar: Option<String>,
    #[serde(rename = "Kamer")]
    pub kamer: Option<i64>,
    #[serde(rename = "Volgnummer")]
    pub volgnummer: Option<i64>,
    #[serde(rename = "Citeertitel")]
    pub citeertitel: Option<String>,
    #[serde(rename = "Alias")]
    pub alias: Option<String>,
    #[serde(rename = "DatumRegistratie")]
    pub datum_registratie: Option<String>,
    #[serde(rename = "DatumOntvangst")]
    pub datum_ontvangst: Option<String>,
    #[serde(rename = "Aanhangselnummer")]
    pub aanhangselnummer: Option<String>,
    #[serde(rename = "KenmerkAfzender")]
    pub kenmerk_afzender: Option<String>,
    #[serde(rename = "Organisatie")]
    pub organisatie: Option<String>,
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
    #[serde(rename = "DocumentActor")]
    pub document_actor: Option<Vec<DocumentActor>>,
    #[serde(rename = "DocumentVersie")]
    pub document_versie: Option<Vec<DocumentVersie>>,
    #[serde(rename = "Activiteit")]
    pub activiteit: Option<Vec<Activiteit>>,
    #[serde(rename = "Agendapunt")]
    pub agendapunt: Option<Vec<Agendapunt>>,
    #[serde(rename = "Kamerstukdossier")]
    pub kamerstukdossier: Option<Vec<Kamerstukdossier>>,
    #[serde(rename = "Zaak")]
    pub zaak: Option<Vec<Zaak>>,
}

impl TkObject for DocumentActor {
    fn entity_set() -> &'static str {
        "DocumentActor"
    }
}

/// DocumentActor
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/documentactor)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-documentactor.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentActor {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Document_Id")]
    pub document_id: Option<TkId>,
    #[serde(rename = "ActorNaam")]
    pub actor_naam: Option<String>,
    #[serde(rename = "ActorFractie")]
    pub actor_fractie: Option<String>,
    #[serde(rename = "Functie")]
    pub functie: Option<String>,
    #[serde(rename = "Relatie")]
    pub relatie: Option<String>,
    #[serde(rename = "SidActor")]
    pub sid_actor: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Fractie_Id")]
    pub fractie_id: Option<TkId>,
    #[serde(rename = "Commissie_Id")]
    pub commissie_id: Option<TkId>,
    #[serde(rename = "Commissie")]
    pub commissie: Option<String>,
    #[serde(rename = "Document")]
    pub document: Option<Document>,
    #[serde(rename = "Fractie")]
    pub fractie: Option<Fractie>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for DocumentVersie {
    fn entity_set() -> &'static str {
        "DocumentVersie"
    }
}

/// DocumentVersie
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/documentversie)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-documentversie.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct DocumentVersie {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Versienummer")]
    pub versienummer: Option<i64>,
    #[serde(rename = "Bestandsgrootte")]
    pub bestandsgrootte: Option<i64>,
    #[serde(rename = "Extensie")]
    pub extensie: Option<String>,
    #[serde(rename = "Datum")]
    pub datum: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Document_Id")]
    pub document_id: Option<TkId>,
    #[serde(rename = "Document")]
    pub document: Option<Document>,
}
