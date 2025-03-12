//! Module met de modellen voor de objecten uit het Gegevensmagazijn van de
//! Tweede Kamer.
//!
//! - [Documentatie](https://opendata.tweedekamer.nl/documentatie)
//! - [XSD](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal/blob/master/xsd/tkData-v1-0.xsd)

use serde::{Deserialize, Serialize};

mod activiteit;
mod agendapunt;
mod besluit;
mod commissie;
mod document;
mod fractie;
mod kamerstukdossier;
mod persoon;
mod reservering;
mod stemming;
mod vergadering;
mod zaak;
mod zaal;

pub use activiteit::*;
pub use agendapunt::*;
pub use besluit::*;
pub use commissie::*;
pub use document::*;
pub use fractie::*;
pub use kamerstukdossier::*;
pub use persoon::*;
pub use reservering::*;
pub use stemming::*;
pub use vergadering::*;
pub use zaak::*;
pub use zaal::*;

/// Struct dat een Entiteit zijn ID vertegenwoordigt
#[derive(
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Default,
    Serialize,
    Deserialize,
)]
pub struct TkId(String);

/// Implementatie van `TkId`
impl TkId {
    /// Create a new TkId from a string
    pub fn new<S: Into<String>>(id: S) -> Self {
        TkId(id.into())
    }

    /// Get the ID as a string reference
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to a owned String
    pub fn into_string(self) -> String {
        self.0
    }
}

/// Implementatie van `From<&str>`` voor `TkId`
impl From<&str> for TkId {
    fn from(s: &str) -> Self {
        TkId(s.to_string())
    }
}

/// Impl `From<String>`` for `TkId`
impl From<String> for TkId {
    fn from(s: String) -> Self {
        TkId(s)
    }
}

/// Implementatie van `AsRef<str>` voor `TkId`
impl AsRef<str> for TkId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Implementatie van `Display` voor `TkId`
impl std::fmt::Display for TkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Trait dat een object uit het Gegevensmagazijn vertegenwoordigt.
/// Dit is een sealed trait - kan alleen worden geïmplementeerd binnen deze
/// crate.
///
/// # Functies
///
/// * `entity_set()` - statische string met de waarde voor de API
pub trait TkObject: serde::de::DeserializeOwned + Serialize {
    /// Get the entity set name for API requests
    fn entity_set() -> &'static str;
}

/// Trait dat een object uit het Gegevensmagazijn vertegenwoordigt
/// waarvoor mogelijk een bestand beschikbaar is.
/// Dit is een sealed trait - kan alleen worden geïmplementeerd binnen deze
/// crate.
pub trait TkResource: TkObject {}
