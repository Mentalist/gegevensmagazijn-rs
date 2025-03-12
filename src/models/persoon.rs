use serde::{Deserialize, Serialize};

use super::{
    ActiviteitActor, CommissieZetelVastPersoon, CommissieZetelVervangerPersoon,
    DocumentActor, FractieZetelPersoon, Stemming, TkId, TkObject, TkResource,
    ZaakActor,
};

impl TkResource for Persoon {}

impl TkObject for Persoon {
    fn entity_set() -> &'static str {
        "Persoon"
    }
}

/// Persoon
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persoon)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persoon.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Persoon {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Nummer")]
    pub nummer: Option<i64>,
    #[serde(rename = "Titels")]
    pub titels: Option<String>,
    #[serde(rename = "Initialen")]
    pub initialen: Option<String>,
    #[serde(rename = "Tussenvoegsel")]
    pub tussenvoegsel: Option<String>,
    #[serde(rename = "Achternaam")]
    pub achternaam: Option<String>,
    #[serde(rename = "Voornamen")]
    pub voornamen: Option<String>,
    #[serde(rename = "Roepnaam")]
    pub roepnaam: Option<String>,
    #[serde(rename = "Geslacht")]
    pub geslacht: Option<String>,
    #[serde(rename = "Functie")]
    pub functie: Option<String>,
    #[serde(rename = "Geboortedatum")]
    pub geboortedatum: Option<String>,
    #[serde(rename = "Geboorteplaats")]
    pub geboorteplaats: Option<String>,
    #[serde(rename = "Geboorteland")]
    pub geboorteland: Option<String>,
    #[serde(rename = "Overlijdensdatum")]
    pub overlijdensdatum: Option<String>,
    #[serde(rename = "Overlijdensplaats")]
    pub overlijdensplaats: Option<String>,
    #[serde(rename = "Woonplaats")]
    pub woonplaats: Option<String>,
    #[serde(rename = "Land")]
    pub land: Option<String>,
    #[serde(rename = "Fractielabel")]
    pub fractielabel: Option<String>,
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
    #[serde(rename = "CommissieZetelVastPersoon")]
    pub commissie_zetel_vast_persoon: Option<Vec<CommissieZetelVastPersoon>>,
    #[serde(rename = "CommissieZetelVervangerPersoon")]
    pub commissie_zetel_vervanger_persoon:
        Option<Vec<CommissieZetelVervangerPersoon>>,
    #[serde(rename = "DocumentActor")]
    pub document_actor: Option<Vec<DocumentActor>>,
    #[serde(rename = "FractieZetelPersoon")]
    pub fractie_zetel_persoon: Option<Vec<FractieZetelPersoon>>,
    #[serde(rename = "PersoonContactinformatie")]
    pub persoon_contactinformatie: Option<Vec<PersoonContactinformatie>>,
    #[serde(rename = "PersoonGeschenk")]
    pub persoon_geschenk: Option<Vec<PersoonGeschenk>>,
    #[serde(rename = "PersoonLoopbaan")]
    pub persoon_loopbaan: Option<Vec<PersoonLoopbaan>>,
    #[serde(rename = "PersoonNevenfunctie")]
    pub persoon_nevenfunctie: Option<Vec<PersoonNevenfunctie>>,
    #[serde(rename = "PersoonOnderwijs")]
    pub persoon_onderwijs: Option<Vec<PersoonOnderwijs>>,
    #[serde(rename = "PersoonReis")]
    pub persoon_reis: Option<Vec<PersoonReis>>,
    #[serde(rename = "Stemming")]
    pub stemming: Option<Vec<Stemming>>,
    #[serde(rename = "ZaakActor")]
    pub zaak_actor: Option<Vec<ZaakActor>>,
}

impl TkObject for PersoonContactinformatie {
    fn entity_set() -> &'static str {
        "PersoonContactinformatie"
    }
}

/// PersoonContactinformatie
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persooncontactinformatie)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persooncontactinformatie.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PersoonContactinformatie {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Soort")]
    pub soort: Option<String>,
    #[serde(rename = "Waarde")]
    pub waarde: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for PersoonGeschenk {
    fn entity_set() -> &'static str {
        "PersoonGeschenk"
    }
}

/// PersoonGeschenk
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persoongeschenk)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persoongeschenk.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PersoonGeschenk {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "Datum")]
    pub datum: Option<String>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for PersoonLoopbaan {
    fn entity_set() -> &'static str {
        "PersoonLoopbaan"
    }
}

/// PersoonLoopbaan
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persoonloopbaan)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persoonloopbaan.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PersoonLoopbaan {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Functie")]
    pub functie: Option<String>,
    #[serde(rename = "Werkgever")]
    pub werkgever: Option<String>,
    #[serde(rename = "OmschrijvingNl")]
    pub omschrijving_nl: Option<String>,
    #[serde(rename = "OmschrijvingEn")]
    pub omschrijving_en: Option<String>,
    #[serde(rename = "Plaats")]
    pub plaats: Option<String>,
    #[serde(rename = "Van")]
    pub van: Option<String>,
    #[serde(rename = "TotEnMet")]
    pub tot_en_met: Option<String>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for PersoonNevenfunctie {
    fn entity_set() -> &'static str {
        "PersoonNevenfunctie"
    }
}

/// PersoonNevenfunctie
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persoonnevenfunctie)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persoonnevenfunctie.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PersoonNevenfunctie {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "PersoonId")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Omschrijving")]
    pub omschrijving: Option<String>,
    #[serde(rename = "PeriodeVan")]
    pub periode_van: Option<String>,
    #[serde(rename = "PeriodeTotEnMet")]
    pub periode_tot_en_met: Option<String>,
    #[serde(rename = "IsActief")]
    pub is_actief: Option<bool>,
    #[serde(rename = "VergoedingSoort")]
    pub vergoeding_soort: Option<String>,
    #[serde(rename = "VergoedingToelichting")]
    pub vergoeding_toelichting: Option<String>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
    #[serde(rename = "PersoonNevenfunctieInkomsten")]
    pub persoon_nevenfunctie_inkomsten:
        Option<Vec<PersoonNevenfunctieInkomsten>>,
}

impl TkObject for PersoonNevenfunctieInkomsten {
    fn entity_set() -> &'static str {
        "PersoonNevenfunctieInkomsten"
    }
}

/// PersoonNevenfunctieInkomsten
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persoonnevenfunctieinkomsten)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persoonnevenfunctieinkomsten.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PersoonNevenfunctieInkomsten {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Jaar")]
    pub jaar: Option<String>,
    #[serde(rename = "BedragSoort")]
    pub bedrag_soort: Option<String>,
    #[serde(rename = "BedragVoorvoegsel")]
    pub bedrag_voorvoegsel: Option<String>,
    #[serde(rename = "BedragValuta")]
    pub bedrag_valuta: Option<String>,
    #[serde(rename = "Bedrag")]
    pub bedrag: Option<f64>,
    #[serde(rename = "BedragAchtervoegsel")]
    pub bedrag_achtervoegsel: Option<String>,
    #[serde(rename = "Frequentie")]
    pub frequentie: Option<String>,
    #[serde(rename = "FrequentieBeschrijving")]
    pub frequentie_beschrijving: Option<String>,
    #[serde(rename = "Opmerking")]
    pub opmerking: Option<String>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Nevenfunctie_Id")]
    pub nevenfunctie_id: Option<TkId>,
    #[serde(rename = "PersoonNevenfunctie")]
    pub persoon_nevenfunctie: Option<PersoonNevenfunctie>,
}

impl TkObject for PersoonOnderwijs {
    fn entity_set() -> &'static str {
        "PersoonOnderwijs"
    }
}

/// PersoonOnderwijs
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persoononderwijs)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persoononderwijs.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PersoonOnderwijs {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "OpleidingNl")]
    pub opleiding_nl: Option<String>,
    #[serde(rename = "OpleidingEn")]
    pub opleiding_en: Option<String>,
    #[serde(rename = "Instelling")]
    pub instelling: Option<String>,
    #[serde(rename = "Plaats")]
    pub plaats: Option<String>,
    #[serde(rename = "Van")]
    pub van: Option<String>,
    #[serde(rename = "TotEnMet")]
    pub tot_en_met: Option<String>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}

impl TkObject for PersoonReis {
    fn entity_set() -> &'static str {
        "PersoonReis"
    }
}

/// PersoonReis
///
/// - [Documentatie](https://opendata.tweedekamer.nl/documentatie/persoonreis)
/// - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0-persoonreis.xsd)
#[derive(
    Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct PersoonReis {
    #[serde(rename = "Id")]
    pub id: Option<TkId>,
    #[serde(rename = "Doel")]
    pub doel: Option<String>,
    #[serde(rename = "Bestemming")]
    pub bestemming: Option<String>,
    #[serde(rename = "Van")]
    pub van: Option<String>,
    #[serde(rename = "TotEnMet")]
    pub tot_en_met: Option<String>,
    #[serde(rename = "BetaaldDoor")]
    pub betaald_door: Option<String>,
    #[serde(rename = "Gewicht")]
    pub gewicht: Option<i64>,
    #[serde(rename = "GewijzigdOp")]
    pub gewijzigd_op: Option<String>,
    #[serde(rename = "ApiGewijzigdOp")]
    pub api_gewijzigd_op: Option<String>,
    #[serde(rename = "Verwijderd")]
    pub verwijderd: Option<bool>,
    #[serde(rename = "Persoon_Id")]
    pub persoon_id: Option<TkId>,
    #[serde(rename = "Persoon")]
    pub persoon: Option<Persoon>,
}
