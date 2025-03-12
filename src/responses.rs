//! Module met structs die de antwoorden van de API vertegenwoordigen.

use std::{
    error, fmt,
    iter::{FromIterator, IntoIterator},
};

use serde::{Deserialize, Serialize};

/// Struct die een 200 OK-antwoord van de API vertegenwoordigt wanneer een
/// enkelvoudig object wordt aangevraagd.
///
/// # Argumenten
///
/// * `T` - type dat `models::Object` implementeert
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
#[serde(rename_all = "camelCase")]
pub struct SingularResponse<T> {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(flatten)]
    pub value: T,
}

/// Struct die een 200 OK-antwoord van de API vertegenwoordigt wanneer meerdere
/// van een object worden aangevraagd.
///
/// # Argumenten
///
/// * `T` - type dat `models::TkObject` implementeert
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
#[serde(rename_all = "camelCase")]
pub struct VectorResponse<T> {
    #[serde(rename = "@odata.context")]
    pub odata_context: String,
    #[serde(rename = "@odata.count")]
    pub odata_count: Option<i64>,
    pub value: Vec<T>,
    #[serde(rename = "@odata.nextLink")]
    pub odata_next_link: Option<String>,
}

impl<T> VectorResponse<T> {
    /// Create a new empty VectorResponse
    pub fn new() -> Self {
        Self {
            odata_context: String::new(),
            odata_count: None,
            value: Vec::new(),
            odata_next_link: None,
        }
    }

    /// Return the number of items in the response
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Check if the response has no items
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl<T> FromIterator<T> for VectorResponse<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let value: Vec<T> = iter.into_iter().collect();
        let count = value.len() as i64;

        Self {
            odata_context: String::new(),
            odata_count: Some(count),
            value,
            odata_next_link: None,
        }
    }
}

impl<T> Extend<T> for VectorResponse<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.value.extend(iter);
        if let Some(count) = &mut self.odata_count {
            *count = self.value.len() as i64;
        } else {
            self.odata_count = Some(self.value.len() as i64);
        }
    }
}

impl<T> IntoIterator for VectorResponse<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

/// Struct die een resource download van de API vertegenwoordigt wanneer een
/// deze succesvol is.
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
#[serde(rename_all = "camelCase")]
pub struct ResourceResponse {
    pub path: String,
    pub filename: String,
}

impl fmt::Display for ResourceResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.filename)
    }
}

/// Struct die een foutreactie van de API vertegenwoordigt wanneer een verzoek
/// wordt uitgevoerd.
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
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: ErrorResponseValue,
}

impl error::Error for ErrorResponse {}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}

/// Struct die de waarde vertegenwoordigt van een foutreactie van de API wanneer
/// een verzoek wordt uitgevoerd.
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
#[serde(rename_all = "camelCase")]
pub struct ErrorResponseValue {
    pub code: String,
    pub message: String,
}

impl fmt::Display for ErrorResponseValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.code, self.message)
    }
}
