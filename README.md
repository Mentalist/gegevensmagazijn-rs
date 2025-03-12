# `gegevensmagazijn`

Een rudimentaire, simplistische en generieke Rust-implementatie van een API-client voor het [Gegevensmagazijn](https://gegevensmagazijn.tweedekamer.nl/) ([OData API](https://opendata.tweedekamer.nl/documentatie/odata-api)) van de [Tweede Kamer der Staten-Generaal](https://www.tweedekamer.nl/).

## Installatie

Om deze Client te gebruiken dien je hem toe te voegen aan je project. Dit kan door de broncode te klonen of direct te verwijzen naar de GitHub-repo.

```shell
$ git clone https://github.com/Mentalist/gegevensmagazijn-rs.git
```

Voeg `gegevensmagazijn` met het pad naar de broncode en [`tokio`](https://github.com/tokio-rs/tokio) als volgt toe aan je `Cargo.toml`:

```toml
[dependencies]
gegevensmagazijn = { version = "0.1.0", path = "/pad/naar/broncode" } 
# gegevensmagazijn = { git = "https://github.com/Mentalist/gegevensmagazijn-rs.git" } 
tokio = { version = "1", features = ["full"] }
```

Success!

## Gebruik

Het initialiseren van een Client is niet moeilijk, de enige keuze is tussen `new()` en `new_with_base_url(base_url: &str)`:

```rust
use gegevensmagazijn::{Client, ClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new()?;
    /* of: */
    let client = Client::new_with_base_url("http://cache-of-mock-server.local/")?;
    /* of: */
    let client = ClientBuilder::new()
        .base_url("http://cache-of-mock-server.local/")
        .build()?;
    /* of: */
    let client = Client::builder()
        .base_url("http://cache-of-mock-server.local/")
        .user_agent("niet firefox")
        .build()?;
    Ok(())
}
```

### Voorbeelden

Er zijn verschillende (**lees**: te veel, *maar ach, waarom niet?*) manieren beschikbaar om gegevens op te halen uit het Gegevensmagazijn via de Client.

* `get_singular<T: Object>(id: &str, query: &str)`
    <details>
      <summary>Voorbeeld</summary>

        let result = client.get_singular::<Activiteit>(
            "b7cf7dfc-1f46-4351-b2a5-000236b07ffa",
            "$select=Id,Onderwerp",
        ).await;
    </details>


* `get_singular_with_response<T: Object>(id: &str, query: &str)`
    <details>
      <summary>Voorbeeld</summary>

        let result = client.get_singular_with_response::<Activiteit>(
            "b7cf7dfc-1f46-4351-b2a5-000236b07ffa",
            "$select=Id,Onderwerp",
        ).await;
    </details>


* `get_vector<T: Object>(query: &str)`
    <details>
        <summary>Voorbeeld</summary>

        let result = client.get_vector::<Besluit>(
        "$filter=Verwijderd eq false and (BesluitSoort eq 'Stemmen - aangenomen' or BesluitSoort eq 'Stemmen - verworpen') and GewijzigdOp ge 2022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp asc &$expand=Zaak($filter=Verwijderd eq false;$expand=ZaakActor($filter=Relatie eq 'Indiener' or Relatie eq 'Medeindiener' and Verwijderd eq false),Document($filter=Verwijderd eq false)),Stemming($filter=Verwijderd eq false),Agendapunt($filter=Verwijderd eq false;$expand=Activiteit($filter=Soort eq 'Stemmingen'))",
        ).await;
    </details>


* `get_vector_recursive<T: Object>(query: &str, max: Option<u32>)`
    <details>
        <summary>Voorbeeld</summary>

        let result = client.get_vector_recursive::<Besluit>(
            "$filter=Verwijderd eq false and (BesluitSoort eq 'Stemmen - aangenomen' or BesluitSoort eq 'Stemmen - verworpen') and GewijzigdOp ge 2022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp asc &$expand=Zaak($filter=Verwijderd eq false;$expand=ZaakActor($filter=Relatie eq 'Indiener' or Relatie eq 'Medeindiener' and Verwijderd eq false),Document($filter=Verwijderd eq false)),Stemming($filter=Verwijderd eq false),Agendapunt($filter=Verwijderd eq false;$expand=Activiteit($filter=Soort eq 'Stemmingen'))",
            Some(1),
        ).await;
    </details>


* `get_vector_with_response<T: Object>(query: &str)`
    <details>
        <summary>Voorbeeld</summary>

        let result = client.get_vector_with_response::<Besluit>(
            "$filter=Verwijderd eq false and (BesluitSoort eq 'Stemmen - aangenomen' or BesluitSoort eq 'Stemmen - verworpen') and GewijzigdOp ge 2022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp asc &$expand=Zaak($filter=Verwijderd eq false;$expand=ZaakActor($filter=Relatie eq 'Indiener' or Relatie eq 'Medeindiener' and Verwijderd eq false),Document($filter=Verwijderd eq false)),Stemming($filter=Verwijderd eq false),Agendapunt($filter=Verwijderd eq false;$expand=Activiteit($filter=Soort eq 'Stemmingen'))",
        ).await;
    </details>


* `get_resource<T: Resource>(id: &str, dir: &str)`
    <details>
        <summary>Voorbeeld</summary>

        let result = client.get_resource::<Document>(
            "2470914b-8978-4dc1-bc7e-c999e2671d21",
            "tests/tmp/",
        ).await;
    </details>

## Testen

Om een werkende Client enigszins, maar toch niet te garanderen, zijn een aantal tests opgenomen om te valideren of deze voldoet aan de acceptatiecriteria.

Gebruik `cargo test` om alle tests uit te voeren.

# Referenties

* [Rust](https://www.rust-lang.org/)
* [Tokio](https://tokio.rs/)
* [Serde](https://serde.rs/)
* [reqwest](https://github.com/seanmonstar/reqwest)
---
* [Tweede Kamer der Staten-Generaal](https://www.tweedekamer.nl/)
* [Gegevensmagazijn](https://gegevensmagazijn.tweedekamer.nl/)
* [Open Data Portaal/OData API](https://opendata.tweedekamer.nl/documentatie/odata-api)
* [GitHub: TweedeKamerDerStaten-Generaal/OpenDataPortaal](https://github.com/TweedeKamerDerStaten-Generaal/OpenDataPortaal)


# Bijdragen

Bijdragen zijn welkom indien ze van hoge kwaliteit en gedetailleerd zijn en helpen de code en gebruikerservaring van deze crate te verbeteren. Het vooraf aanmaken van een issue wordt op prijs gesteld; zorg ervoor dat commits zijn gesigneerd.

# Licentie

`gegevensmagazijn` is gratis en open source software, vrijgegeven onder de voorwaarden van de [MIT-licentie](./LICENSE).
