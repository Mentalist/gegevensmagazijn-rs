use std::{
    fs::File,
    io::{Cursor, copy},
};

use reqwest::{Client as Http, Url};
use serde::de::DeserializeOwned;

use crate::{
    error::{GmError, api_error_from_status, invalid_input_error},
    models::{TkObject, TkResource},
    responses,
    responses::{ResourceResponse, SingularResponse, VectorResponse},
};

/// Een rudimentaire, simplistische en generieke Rust-implementatie van een
/// API-client voor het Gegevensmagazijn (OData API) van de Tweede Kamer der
/// Staten-Generaal.
#[derive(Clone)]
pub struct Client {
    client: Http,
    base_url: Url,
    user_agent: String,
}

/// Configuratieopties voor de API-client
#[derive(Debug, Clone)]
pub struct ClientOptions {
    /// User-Agent string voor API-verzoeken
    pub user_agent: String,
    /// Basis-URL voor de API
    pub base_url: String,
}

/// Builder voor het configureren van de API-client
pub struct ClientBuilder {
    options: ClientOptions,
}

/// Implementatie van `Client`
impl Client {
    /// De officiële OData API URL van het Gegevensmagazijn van de Tweede Kamer
    /// der Staten-Generaal. Gebruik een cache- of mockserver bij testen
    /// en/of een integratie met < < veel > > verkeer
    pub const DEFAULT_BASE_URL: &'static str =
        "https://gegevensmagazijn.tweedekamer.nl/OData/v4/2.0/";

    /// Creëert een builder voor het configureren van de client
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::Client;
    ///
    /// let client = Client::builder()
    ///     .base_url("https://mijn-cache-server.nl/api/")
    ///     .user_agent("Mijn App/1.0")
    ///     .build()
    ///     .expect("Client maken mislukt");
    /// ```
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Creëert een instantie van `Client` met de waarde van `DEFAULT_BASE_URL`
    /// en retourneert deze of een fout.
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::Client;
    ///
    /// let client = Client::new();
    ///
    /// assert!(client.is_ok())
    /// ```
    pub fn new() -> crate::error::Result<Self> {
        Self::new_with_base_url(Self::DEFAULT_BASE_URL)
    }

    /// Creëert een instantie van `Client` met de opgegeven waarde van parameter
    /// `base url` en retourneert deze of een fout.
    ///
    /// # Argumenten
    ///
    /// * `base_url` - string met de te gebruiken API-url
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::Client;
    ///
    /// let client = Client::new_with_base_url(
    ///     "https://cache-of-mockserver.nl/OData/v4/2.0/",
    /// );
    ///
    /// assert!(client.is_ok())
    /// ```
    pub fn new_with_base_url(base_url: &str) -> crate::error::Result<Self> {
        let client = Http::new();
        let base_url = Url::parse(base_url).map_err(GmError::from)?;

        Ok(Self {
            client,
            base_url,
            user_agent: format!(
                "gegevensmagazijn rust library/{}",
                env!("CARGO_PKG_VERSION")
            ),
        })
    }

    /// Retourneert de waarde van `base_url`.
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::{Client, error::Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::new()?;
    ///
    ///     assert_eq!(client.base_url(), Client::DEFAULT_BASE_URL);
    ///     Ok(())
    /// }
    /// ```
    pub fn base_url(&self) -> &str {
        self.base_url.as_str()
    }

    /// Creëert een URL volgens de opgegeven waarden, voert het juiste verzoek
    /// uit en retourneert een enkele `T`.
    ///
    /// # Argumenten
    ///
    /// * `T` - type dat `models::TkObject` implementeert
    /// * `id` - string die de id van `T` op de API bevat
    /// * `query` - string die `?` URL-query voor de API bevat
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::{Client, error::Result, models::Agendapunt};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::new()?;
    ///     let response: Agendapunt = client
    ///         .get_singular::<Agendapunt>(
    ///             "21323144-10e7-4010-baee-00085dec6aa3",
    ///             "$select=Id,Onderwerp",
    ///         )
    ///         .await?;
    ///
    ///     assert_eq!(
    ///         response.onderwerp,
    ///         Some(String::from(
    ///             "Financiering van Nederlandse infrastructuur door \
    ///              institutionele beleggers "
    ///         ))
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_singular<T: TkObject>(
        &self,
        id: &str,
        query: &str,
    ) -> crate::error::Result<T> {
        if id.is_empty() {
            return Err(invalid_input_error("ID kan niet leeg zijn"));
        }

        let mut url = Url::parse(&format!(
            "{}{}({})",
            self.base_url,
            T::entity_set(),
            id
        ))
        .map_err(GmError::from)?;

        url.set_query(Some(query));
        let response = self.execute_request::<SingularResponse<T>>(url).await?;
        Ok(response.value)
    }

    /// Creëert een URL volgens de opgegeven waarden, voert het juiste verzoek
    /// uit en retourneert een enkele `T` in een `SingularResponse<T>`.
    ///
    /// # Argumenten
    ///
    /// * `T` - type dat `models::TkObject` implementeert
    /// * `id` - string die de id van `T` op de API bevat
    /// * `query` - string die `?` URL-query voor de API bevat
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::{
    ///     Client, error::Result, models::PersoonContactinformatie,
    ///     responses::SingularResponse,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::new()?;
    ///     let response: SingularResponse<PersoonContactinformatie> = client
    ///         .get_singular_with_response::<PersoonContactinformatie>(
    ///             "ab86e0bc-b2de-4797-a84d-00392e14b5c3",
    ///             "",
    ///         )
    ///         .await?;
    ///
    ///     assert_eq!(
    ///         response.value.waarde,
    ///         Some(String::from("stephan-van-baarle-a5014763"))
    ///     );
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_singular_with_response<T: TkObject>(
        &self,
        id: &str,
        query: &str,
    ) -> crate::error::Result<SingularResponse<T>> {
        if id.is_empty() {
            return Err(invalid_input_error("ID kan niet leeg zijn"));
        }

        let mut url = Url::parse(&format!(
            "{}{}({})",
            self.base_url,
            T::entity_set(),
            id
        ))
        .map_err(GmError::from)?;

        url.set_query(Some(query));
        let response = self.execute_request::<SingularResponse<T>>(url).await?;
        Ok(response)
    }

    /// Creëert een URL volgens de opgegeven waarden, voert het juist verzoek
    /// uit en retourneert een `vec<T>`.
    ///
    /// # Argumenten
    ///
    /// * `T` - type dat `models::TkObject` implementeert
    /// * `query` - string die `?` URL-query voor de API bevat
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::{Client, error::Result, models::Fractie};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::new()?;
    ///     let response: Vec<Fractie> = client
    ///         .get_vector::<Fractie>("$filter=Verwijderd eq false&$count=true")
    ///         .await?;
    ///
    ///     println!("Aantal fracties: {}", response.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_vector<T: TkObject>(
        &self,
        query: &str,
    ) -> crate::error::Result<Vec<T>> {
        let mut url =
            Url::parse(&format!("{}{}", self.base_url, T::entity_set()))
                .map_err(GmError::from)?;

        url.set_query(Some(query));
        let response = self
            .execute_request::<VectorResponse<T>>(url.clone())
            .await?;

        Ok(response.value)
    }

    /// Creëert een URL volgens de opgegeven waarden, voert de juiste verzoeken
    /// uit en retourneert een `Vec<T>`.
    ///
    /// # Argumenten
    ///
    /// * `T` - type dat `models::TkObject` implementeert
    /// * `query` - string die `?` URL-query voor de API bevat
    /// * `limit` - recursielimiet configuratie die bepaalt hoeveel pagina's
    ///   moeten worden opgehaald
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::{Client, error::Result, models::Besluit};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::new()?;
    ///
    ///     // Haal eerste pagina plus 2 aanvullende pagina's op
    ///     let limited_response: Vec<Besluit> = client
    ///         .get_vector_recursive::<Besluit>("$count=true", Some(2))
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_vector_recursive<T: TkObject>(
        &self,
        query: &str,
        limit: Option<u32>,
    ) -> crate::error::Result<Vec<T>> {
        let mut url =
            Url::parse(&format!("{}{}", self.base_url, T::entity_set()))
                .map_err(GmError::from)?;

        url.set_query(Some(query));
        let mut values = Vec::new();
        let mut pages_fetched = 0;

        loop {
            let response = self
                .execute_request::<VectorResponse<T>>(url.clone())
                .await?;

            values.extend(response.value);

            if response.odata_next_link.is_none() {
                break;
            }

            match limit {
                Some(max) if pages_fetched >= max => break,
                _ => {
                    url = Url::parse(&response.odata_next_link.unwrap())
                        .map_err(GmError::from)?;
                    pages_fetched += 1;
                },
            }
        }

        Ok(values)
    }

    /// Creëert een URL volgens de opgegeven waarden, voert het juiste verzoek
    /// uit en retourneert een `Vec<T>` in een `VectorResponse<T>`.
    ///
    /// # Argumenten
    ///
    /// * `T` - type dat `models::TkObject` implementeert
    /// * `query` - string die `?` URL-query voor de API bevat
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use gegevensmagazijn::{
    ///     Client, error::Result, models::Persoon, responses::VectorResponse,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::new()?;
    ///     let response: VectorResponse<Persoon> = client
    ///         .get_vector_with_response::<Persoon>("$top=5&$count=true")
    ///         .await?;
    ///
    ///     println!("Aantal personen: {}", response.value.len());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_vector_with_response<T: TkObject>(
        &self,
        query: &str,
    ) -> crate::error::Result<VectorResponse<T>> {
        let mut url =
            Url::parse(&format!("{}{}", self.base_url, T::entity_set()))
                .map_err(GmError::from)?;

        url.set_query(Some(query));
        let response = self.execute_request::<VectorResponse<T>>(url).await?;

        Ok(response)
    }

    /// Creëert een URL volgens de opgegeven waarden, voert het juiste verzoek
    /// uit en retourneert een `ResourceResponse`.
    ///
    /// # Argumenten
    ///
    /// * `T` - type dat `models::TkResource` implementeert
    /// * `id` - string die id van `T` bevat
    /// * `dir` - string die de uitvoermap bevat
    ///
    /// # Voorbeeld
    ///
    /// ```rust
    /// use std::fs;
    ///
    /// use gegevensmagazijn::{Client, error::Result, models::Document};
    /// ```
    pub async fn get_resource<T: TkResource>(
        &self,
        id: &str,
        dir: &str,
    ) -> crate::error::Result<ResourceResponse> {
        if id.is_empty() {
            return Err(invalid_input_error("ID kan niet leeg zijn"));
        }

        let url = Url::parse(&format!(
            "{}{}({})/resource",
            self.base_url,
            T::entity_set(),
            id
        ))
        .map_err(GmError::from)?;

        let response = self
            .client
            .get(url)
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .send()
            .await
            .map_err(GmError::from)?;

        let status = response.status();
        if !status.is_success() {
            return Err(api_error_from_status(
                status,
                "Resource-aanvraag mislukt",
            ));
        }

        let file_name = response
            .headers()
            .get("Content-Disposition")
            .ok_or_else(|| {
                GmError::Resource(
                    "Content-Disposition header ontbreekt".to_string(),
                )
            })?
            .to_str()
            .map_err(|_| {
                GmError::Resource(
                    "Ongeldige Content-Disposition header".to_string(),
                )
            })?;

        let file_name_parts: Vec<&str> = file_name.split('=').collect();
        if file_name_parts.len() < 2 {
            return Err(GmError::Resource(
                "Ongeldig Content-Disposition formaat".to_string(),
            ));
        }

        let file_name = file_name_parts[1].to_owned();
        let file_path = format!("{}{}", dir, file_name);

        let mut file = File::create(&file_path).map_err(|e| {
            GmError::Resource(format!("Bestand aanmaken mislukt: {}", e))
        })?;

        let mut content =
            Cursor::new(response.bytes().await.map_err(GmError::from)?);
        copy(&mut content, &mut file).map_err(|e| {
            GmError::Resource(format!("Bestand schrijven mislukt: {}", e))
        })?;

        Ok(ResourceResponse {
            path: format!("{}{}", dir, file_name),
            filename: file_name,
        })
    }

    /// Voert een verzoek uit met de opgegeven `url`-waarde en retourneert een
    /// instantie van `T` of een fout.
    ///
    /// # Argumenten
    ///
    /// * `T` - type
    /// * `url` - `Url` met de URL voor de te verwerken request
    async fn execute_request<T>(&self, url: Url) -> crate::error::Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .get(url)
            .header(reqwest::header::USER_AGENT, &self.user_agent)
            .send()
            .await
            .map_err(GmError::from)?;

        let status = response.status();
        let response_text = response.text().await.map_err(GmError::from)?;

        if status.is_success() {
            let value: T =
                serde_json::from_str(&response_text).map_err(GmError::from)?;
            Ok(value)
        } else if response_text.is_empty() {
            Err(api_error_from_status(status, "Verzoek mislukt"))
        } else {
            let error = serde_json::from_str::<responses::ErrorResponse>(
                &response_text,
            )
            .map_err(GmError::from)?;

            Err(GmError::from(error))
        }
    }
}

/// Implementatie van `ClientBuilder`
impl ClientBuilder {
    /// Maakt een nieuwe instantie van `ClientBuilder`
    pub fn new() -> Self {
        Self {
            options: ClientOptions::default(),
        }
    }

    /// Stelt een aangepaste basis-URL in voor API-verzoeken
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.options.base_url = url.into();
        self
    }

    /// Stelt een aangepaste User-Agent in voor API-verzoeken
    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.options.user_agent = agent.into();
        self
    }

    /// Bouwt een instantie van `Client` met de opgegeven configuratie
    pub fn build(self) -> crate::error::Result<Client> {
        let client = Http::new();
        let base_url =
            Url::parse(&self.options.base_url).map_err(GmError::from)?;

        Ok(Client {
            client,
            base_url,
            user_agent: self.options.user_agent,
        })
    }
}

/// Standaardwaarden voor de configuratieopties van de API-client
impl Default for ClientOptions {
    fn default() -> Self {
        Self {
            user_agent: format!(
                "gegevensmagazijn rust library/{}",
                env!("CARGO_PKG_VERSION")
            ),
            base_url: Client::DEFAULT_BASE_URL.to_string(),
        }
    }
}

/// Standaardwaarden voor de bouwer van de API-client
impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
