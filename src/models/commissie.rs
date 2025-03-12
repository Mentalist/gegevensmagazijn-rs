use serde::{Deserialize, Serialize};

use super::{
    ActiviteitActor, DocumentActor, Fractie, Persoon, TkId, TkObject, ZaakActor,
};

impl TkObject for Commissie {
    fn entity_set() -> &'static str {
        "Commissie"
    }
}

/// Commissie
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/commissie)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-commissie.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Commissie {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<i64>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "Afkorting")]
    pub afkorting: Option<String>,
    #[serde(rename = "NaamNL")]
    pub naam_nl: Option<String>,
    #[serde(rename = "NaamEN")]
    pub naam_en: Option<String>,
    #[serde(rename = "NaamWebNL")]
    pub naam_web_nl: Option<String>,
    #[serde(rename = "NaamWebEN")]
    pub naam_web_en: Option<String>,
    #[serde(rename = "Inhoudsopgave")]
    pub inhoudsopgave: Option<String>,
    #[serde(rename = "DatumActief")]
    pub datum_actief: Option<String>,
    #[serde(rename = "DatumInactief")]
    pub datum_inactief: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "ActiviteitActor")]
    pub activiteit_actor: Option<Vec<ActiviteitActor>>,
    #[serde(rename = "CommissieContactinformatie")]
    pub commissie_contactinformatie: Option<Vec<CommissieContactinformatie>>,
    #[serde(rename = "CommissieZetel")]
    pub commissie_zetel: Option<Vec<CommissieZetel>>,
    #[serde(rename = "DocumentActor")]
    pub document_actor: Option<Vec<DocumentActor>>,
    #[serde(rename = "ZaakActor")]
    pub zaak_actor: Option<Vec<ZaakActor>>,
}

impl TkObject for CommissieContactinformatie {
    fn entity_set() -> &'static str {
        "CommissieContactinformatie"
    }
}

/// CommissieContactinformatie
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/commissiecontactinformatie)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-commissiecontactinformatie.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CommissieContactinformatie {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "Waarde")]
    pub waarde: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Commissie_Id")]
    pub commissie_id: Option<TkId>,
    #[serde(rename = "Commissie")]
    pub commissie: Option<Commissie>,
}

impl TkObject for CommissieZetel {
    fn entity_set() -> &'static str {
        "CommissieZetel"
    }
}

/// CommissieZetel
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/commissiezetel)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-commissiezetel.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CommissieZetel {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Commissie_Id")]
    pub commissie_id: Option<TkId>,
    #[serde(rename = "CommissieZetelVastPersoon")]
    pub commissie_zetel_vast_persoon: Option<Vec<CommissieZetelVastPersoon>>,
    #[serde(rename = "CommissieZetelVervangerPersoon")]
    pub commissie_zetel_vervanger_persoon:
        Option<Vec<CommissieZetelVervangerPersoon>>,
    #[serde(rename = "Commissie")]
    pub commissie: Option<Commissie>,
    #[serde(rename = "CommissieZetelVastVacature")]
    pub commissie_zetel_vast_vacature: Option<Vec<CommissieZetelVastVacature>>,
    #[serde(rename = "CommissieZetelVervangerVacature")]
    pub commissie_zetel_vervanger_vacature:
        Option<Vec<CommissieZetelVervangerVacature>>,
}

impl TkObject for CommissieZetelVastPersoon {
    fn entity_set() -> &'static str {
        "CommissieZetelVastPersoon"
    }
}

/// CommissieZetelVastPersoon
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/commissiezetelvastpersoon)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-commissiezetelvastpersoon.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CommissieZetelVastPersoon {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
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
    #[serde(rename = "CommissieZetel_Id")]
    pub commissie_zetel_id: Option<TkId>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "CommissieZetel")]
    pub commissie_zetel: Option<CommissieZetel>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for CommissieZetelVastVacature {
    fn entity_set() -> &'static str {
        "CommissieZetelVastVacature"
    }
}

/// CommissieZetelVastVacature
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/commissiezetelvastvacature)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-commissiezetelvastvacature.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CommissieZetelVastVacature {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
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
    #[serde(rename = "CommissieZetel_Id")]
    pub commissie_zetel_id: Option<TkId>,
    #[serde(rename = "Fractie_Id")]
    pub fractie_id: Option<TkId>,
    #[serde(rename = "Fractie")]
    pub fractie: Option<Fractie>,
    #[serde(rename = "CommissieZetel")]
    pub commissie_zetel: Option<CommissieZetel>,
}

impl TkObject for CommissieZetelVervangerPersoon {
    fn entity_set() -> &'static str {
        "CommissieZetelVervangerPersoon"
    }
}

/// CommissieZetelVervangerPersoon
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/commissiezetelvervangerpersoon)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-commissiezetelvervangerpersoon.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CommissieZetelVervangerPersoon {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
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
    #[serde(rename = "CommissieZetel_Id")]
    pub commissie_zetel_id: Option<TkId>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "CommissieZetel")]
    pub commissie_zetel: Option<CommissieZetel>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for CommissieZetelVervangerVacature {
    fn entity_set() -> &'static str {
        "CommissieZetelVervangerVacature"
    }
}

/// CommissieZetelVervangerVacature
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/commissiezetelvervangervacature)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-commissiezetelvervangervacature.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CommissieZetelVervangerVacature {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
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
    #[serde(rename = "CommissieZetel_Id")]
    pub commissie_zetel_id: Option<TkId>,
    #[serde(rename = "Fractie_Id")]
    pub fractie_id: Option<TkId>,
    #[serde(rename = "CommissieZetel")]
    pub commissie_zetel: Option<CommissieZetel>,
    #[serde(rename = "Fractie")]
    pub fractie: Option<Fractie>,
}
