use serde::{Deserialize, Serialize};

use super::{
    Agendapunt, Commissie, Document, Fractie, Persoon, Reservering, TkId,
    TkObject, Zaak,
};

impl TkObject for Activiteit {
    fn entity_set() -> &'static str {
        "Activiteit"
    }
}

/// Activiteit
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/activiteit)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-activiteit.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Activiteit {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<String>,
    #[serde(rename = "Onderwerp")]
    pub onderwerp: Option<String>,
    #[serde(rename = "DatumSoort")]
    pub datum_soort: Option<String>,
    #[serde(rename = "Datum")]
    pub datum: Option<String>,
    #[serde(rename = "Aanvangstijd")]
    pub aanvangstijd: Option<String>,
    #[serde(rename = "Eindtijd")]
    pub eindtijd: Option<String>,
    #[serde(rename = "Locatie")]
    pub locatie: Option<String>,
    #[serde(rename = "Besloten")]
    pub besloten: Option<bool>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "Vergaderjaar")]
    pub vergaderjaar: Option<String>,
    #[serde(rename = "Kamer")]
    pub kamer: Option<String>,
    #[serde(rename = "Noot")]
    pub noot: Option<String>,
    #[serde(rename = "VRSNummer")]
    pub vrsnummer: Option<String>,
    #[serde(rename = "SidVoortouw")]
    pub sid_voortouw: Option<String>,
    #[serde(rename = "Voortouwnaam")]
    pub voortouwnaam: Option<String>,
    #[serde(rename = "Voortouwafkorting")]
    pub voortouwafkorting: Option<String>,
    #[serde(rename = "Voortouwkortenaam")]
    pub voortouwkortenaam: Option<String>,
    #[serde(rename = "Voortouwcommissie_Id")]
    pub voortouwcommissie_id: Option<TkId>,
    #[serde(rename = "Aanvraagdatum")]
    pub aanvraagdatum: Option<String>,
    #[serde(rename = "DatumVerzoekEersteVerlenging")]
    pub datum_verzoek_eerste_verlenging: Option<String>,
    #[serde(rename = "DatumMededelingEersteVerlenging")]
    pub datum_mededeling_eerste_verlenging: Option<String>,
    #[serde(rename = "DatumVerzoekTweedeVerlenging")]
    pub datum_verzoek_tweede_verlenging: Option<String>,
    #[serde(rename = "DatumMededelingTweedeVerlenging")]
    pub datum_mededeling_tweede_verlenging: Option<String>,
    #[serde(rename = "Vervaldatum")]
    pub vervaldatum: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Document")]
    pub document: Option<Vec<Document>>,
    #[serde(rename = "Reservering")]
    pub reservering: Option<Vec<Reservering>>,
    #[serde(rename = "Zaak")]
    pub zaak: Option<Vec<Zaak>>,
    #[serde(rename = "ActiviteitActor")]
    pub activiteit_actor: Option<Vec<ActiviteitActor>>,
    #[serde(rename = "Agendapunt")]
    pub agendapunt: Option<Vec<Agendapunt>>,
}

impl TkObject for ActiviteitActor {
    fn entity_set() -> &'static str {
        "ActiviteitActor"
    }
}

/// ActiviteitActor
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/activiteitactor)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-activiteitactor.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ActiviteitActor {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Activiteit_Id")]
    pub activiteit_id: Option<TkId>,
    #[serde(rename = "ActorNaam")]
    pub actor_naam: Option<String>,
    #[serde(rename = "ActorFractie")]
    pub actor_fractie: Option<String>,
    #[serde(rename = "Relatie")]
    pub relatie: Option<String>,
    #[serde(rename = "Volgorde")]
    pub volgorde: Option<i64>,
    #[serde(rename = "Functie")]
    pub functie: Option<String>,
    #[serde(rename = "Spreektijd")]
    pub spreektijd: Option<String>,
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
    #[serde(rename = "Activiteit")]
    pub activiteit: Option<Activiteit>,
    #[serde(rename = "Commissie")]
    pub commissie: Option<Commissie>,
    #[serde(rename = "Fractie")]
    pub fractie: Option<Fractie>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}
