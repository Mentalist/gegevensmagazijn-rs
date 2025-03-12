use serde::{Deserialize, Serialize};

use super::{
    ActiviteitActor, DocumentActor, Persoon, Stemming, TkId, TkObject,
    TkResource, ZaakActor,
};

impl TkResource for Fractie {}

impl TkObject for Fractie {
    fn entity_set() -> &'static str {
        "Fractie"
    }
}

/// Fractie
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/fractie)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-fractie.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Fractie {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<i64>,
    #[serde(rename = "Afkorting")]
    pub afkorting: Option<String>,
    #[serde(rename = "NaamNL")]
    pub naam_nl: Option<String>,
    #[serde(rename = "NaamEN")]
    pub naam_en: Option<String>,
    #[serde(rename = "AantalZetels")]
    pub aantal_zetels: Option<i64>,
    #[serde(rename = "AantalStemmen")]
    pub aantal_stemmen: Option<i64>,
    #[serde(rename = "DatumActief")]
    pub datum_actief: Option<String>,
    #[serde(rename = "DatumInactief")]
    pub datum_inactief: Option<String>,
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
    #[serde(rename = "ActiviteitActor")]
    pub activiteit_actor: Option<Vec<ActiviteitActor>>,
    #[serde(rename = "DocumentActor")]
    pub document_actor: Option<Vec<DocumentActor>>,
    #[serde(rename = "FractieZetel")]
    pub fractie_zetel: Option<Vec<FractieZetel>>,
    #[serde(rename = "Stemming")]
    pub stemming: Option<Vec<Stemming>>,
    #[serde(rename = "ZaakActor")]
    pub zaak_actor: Option<Vec<ZaakActor>>,
}

impl TkObject for FractieZetel {
    fn entity_set() -> &'static str {
        "FractieZetel"
    }
}

/// FractieZetel
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/fractiezetel)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-fractiezetel.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FractieZetel {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "Fractie_Id")]
    pub fractie_id: Option<TkId>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Fractie")]
    pub fractie: Option<Fractie>,
    #[serde(rename = "FractieZetelPersoon")]
    pub fractie_zetel_persoon: Option<Vec<FractieZetelPersoon>>,
    #[serde(rename = "FractieZetelVacature")]
    pub fractie_zetel_vacature: Option<Vec<FractieZetelVacature>>,
}

impl TkObject for FractieZetelPersoon {
    fn entity_set() -> &'static str {
        "FractieZetelPersoon"
    }
}

/// FractieZetelPersoon
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/fractiezetelpersoon)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-fractiezetelpersoon.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FractieZetelPersoon {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "FractieZetel_Id")]
    pub fractie_zetel_id: Option<TkId>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Functie")]
    pub functie: Option<String>,
    #[serde(rename = "Van")]
    pub van: Option<String>,
    #[serde(rename = "TotEnMet")]
    pub tot_en_met: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "FractieZetel")]
    pub fractie_zetel: Option<FractieZetel>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for FractieZetelVacature {
    fn entity_set() -> &'static str {
        "FractieZetelVacature"
    }
}

/// FractieZetelVacature
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/fractiezetelvacature)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-fractiezetelvacature.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct FractieZetelVacature {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "FractieZetel_Id")]
    pub fractie_zetel_id: Option<TkId>,
    #[serde(rename = "Functie")]
    pub functie: Option<String>,
    #[serde(rename = "Van")]
    pub van: Option<String>,
    #[serde(rename = "TotEnMet")]
    pub tot_en_met: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "FractieZetel")]
    pub fractie_zetel: Option<FractieZetel>,
}
