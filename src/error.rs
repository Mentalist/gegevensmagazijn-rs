//! Foutafhandeling voor de gegevensmagazijnbibliotheek
//!
//! Deze module bevat de fouttypes en hulproutines die worden gebruikt in de
//! gegevensmagazijnbibliotheek.

use reqwest::StatusCode;
use thiserror::Error;

/// Fouttype voor de gegevensmagazijnbibliotheek
#[derive(Error, Debug)]
pub enum GmError {
    /// API gaf een foutmelding terug
    #[error("API fout: {code} - {message}")]
    Api {
        /// HTTP statuscode
        code: String,
        /// Foutmelding
        message: String,
    },

    /// Fout tijdens HTTP-verzoek
    #[error("HTTP-verzoek mislukt: {0}")]
    Http(#[from] reqwest::Error),

    /// URL-parsing mislukt
    #[error("URL-parsing fout: {0}")]
    Url(#[from] url::ParseError),

    /// I/O-fout opgetreden
    #[error("I/O-fout: {0}")]
    Io(#[from] std::io::Error),

    /// Serialisatie of deserialisatie mislukt
    #[error("Serialisatiefout: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Resource-verwerking mislukt
    #[error("Resource-verwerking mislukt: {0}")]
    Resource(String),

    /// Ongeldige invoerparameters
    #[error("Ongeldige invoer: {0}")]
    InvalidInput(String),

    /// Onverwachte foutconditie
    #[error("Onverwachte fout: {0}")]
    Other(String),
}

/// Een gespecialiseerd Result-type voor gegevensmagazijn-operaties
pub type Result<T> = std::result::Result<T, GmError>;

/// Conversie van ErrorResponse naar GmError
impl From<crate::responses::ErrorResponse> for GmError {
    fn from(err: crate::responses::ErrorResponse) -> Self {
        GmError::Api {
            code: err.error.code,
            message: err.error.message,
        }
    }
}

/// Helpfunctie om een API-fout te maken van een statuscode en optioneel bericht
pub fn api_error_from_status(
    status: StatusCode,
    default_message: &str,
) -> GmError {
    GmError::Api {
        code: status.as_u16().to_string(),
        message: status
            .canonical_reason()
            .unwrap_or(default_message)
            .to_string(),
    }
}

/// Helpfunctie om een ongeldige-invoerfout te maken
pub fn invalid_input_error(message: &str) -> GmError {
    GmError::InvalidInput(message.to_string())
}
