//! Gegevensmagazijn
//!
//! Een rudimentaire, simplistische en generieke Rust-implementatie van een
//! API-client voor het Gegevensmagazijn (OData API) van de Tweede Kamer der
//! Staten-Generaal.
//!
//! # Voorbeeld
//!
//! ```rust
//! use gegevensmagazijn::{Client, models::Agendapunt};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new()?;
//!     let response: Agendapunt = client
//!         .get_singular::<Agendapunt>(
//!             "21323144-10e7-4010-baee-00085dec6aa3",
//!             "",
//!         )
//!         .await?;
//!
//!     assert_eq!(
//!         response.onderwerp,
//!         Some(String::from(
//!             "Financiering van Nederlandse infrastructuur door \
//!              institutionele beleggers "
//!         ))
//!     );
//!
//!     Ok(())
//! }
//! ```
//!
//! # Referenties
//!
//!    * [Rust](https://www.rust-lang.org/)
//!    * [Tokio](https://tokio.rs/)
//!    * [Serde](https://serde.rs/)
//!    * [reqwest](https://github.com/seanmonstar/reqwest)
//!    ---
//!    * [Tweede Kamer der Staten-Generaal](https://www.tweedekamer.nl/)
//!    * [Gegevensmagazijn](https://gegevensmagazijn.tweedekamer.nl/)
//!    * [Open Data Portaal/OData API](https://opendata.tweedekamer.nl/documentatie/odata-api)
//!    * [GitHub: TweedeKamerDerStaten-Generaal/OpenDataPortaal](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal)

#[cfg(test)]
pub mod tests;

mod client;
pub mod error;
pub mod models;
pub mod responses;

pub use client::*;
