use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    Client,
    models::{Activiteit, Besluit, Document, TkId},
};

#[test]
fn test_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Client>();
}

#[test]
fn test_sync() {
    fn assert_sync<T: Sync>() {}
    assert_sync::<Client>();
}

#[test]
fn new_should_create_client_with_default_base_url() {
    let client = Client::new().expect("Failed to create client");
    assert_eq!(client.base_url(), Client::DEFAULT_BASE_URL);
}

#[test]
fn new_with_base_url_should_create_client_with_specified_base_url() {
    let base_url = "https://example.com/odata/v4/2.0/";
    let client = Client::new_with_base_url(base_url).unwrap();
    assert_eq!(client.base_url(), base_url);
}

#[tokio::test]
async fn get_singular_should_return_activiteit_when_request_is_successful() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock("GET", "/Activiteit(b7cf7dfc-1f46-4351-b2a5-000236b07ffa)?")
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/responses/singular.activiteit.json")
        .create_async()
        .await;

    let client = Client::new_with_base_url(server.url().as_str()).unwrap();
    let result = client
        .get_singular::<Activiteit>("b7cf7dfc-1f46-4351-b2a5-000236b07ffa", "")
        .await;

    assert!(result.is_ok());

    let activity = result.unwrap();
    assert_eq!(
        activity.id,
        Some(TkId::from("b7cf7dfc-1f46-4351-b2a5-000236b07ffa"))
    );
    assert_eq!(
        activity.onderwerp.unwrap_or_default(),
        "Procedures en brieven "
    );

    mock.assert_async().await;
}

#[tokio::test]
async fn get_singular_with_response_should_return_activiteit_when_request_is_successful()
 {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/Activiteit(b7cf7dfc-1f46-4351-b2a5-000236b07ffa)?$select=Id,\
             Onderwerp",
        )
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/responses/singular.activiteit.json")
        .create_async()
        .await;

    let client = Client::new_with_base_url(server.url().as_str()).unwrap();
    let result = client
        .get_singular_with_response::<Activiteit>(
            "b7cf7dfc-1f46-4351-b2a5-000236b07ffa",
            "$select=Id,Onderwerp",
        )
        .await;

    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(
        response.value.id,
        Some(TkId::from("b7cf7dfc-1f46-4351-b2a5-000236b07ffa"))
    );
    assert_eq!(
        response.value.onderwerp.unwrap_or_default(),
        "Procedures en brieven "
    );

    mock.assert_async().await;
}

#[tokio::test]
async fn get_vector_should_return_besluiten_when_request_is_successful() {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/Besluit?$filter=Verwijderd%20eq%20false%20and%20(BesluitSoort%\
             20eq%20%27Stemmen%20-%20aangenomen%27%20or%20BesluitSoort%20eq%\
             20%27Stemmen%20-%20verworpen%27)%20and%20GewijzigdOp%20ge%\
             202022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp%20asc%20&\
             $expand=Zaak($filter=Verwijderd%20eq%20false;\
             $expand=ZaakActor($filter=Relatie%20eq%20%27Indiener%27%20or%\
             20Relatie%20eq%20%27Medeindiener%27%20and%20Verwijderd%20eq%\
             20false),Document($filter=Verwijderd%20eq%20false)),\
             Stemming($filter=Verwijderd%20eq%20false),\
             Agendapunt($filter=Verwijderd%20eq%20false;\
             $expand=Activiteit($filter=Soort%20eq%20%27Stemmingen%27))",
        )
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/responses/vec.besluit.1.json")
        .create_async()
        .await;

    let client = Client::new_with_base_url(server.url().as_str()).unwrap();
    let result = client
        .get_vector::<Besluit>(
            "$filter=Verwijderd eq false and (BesluitSoort eq 'Stemmen - \
             aangenomen' or BesluitSoort eq 'Stemmen - verworpen') and \
             GewijzigdOp ge 2022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp \
             asc &$expand=Zaak($filter=Verwijderd eq \
             false;$expand=ZaakActor($filter=Relatie eq 'Indiener' or Relatie \
             eq 'Medeindiener' and Verwijderd eq \
             false),Document($filter=Verwijderd eq \
             false)),Stemming($filter=Verwijderd eq \
             false),Agendapunt($filter=Verwijderd eq \
             false;$expand=Activiteit($filter=Soort eq 'Stemmingen'))",
        )
        .await;

    assert!(result.is_ok());

    let besluiten = result.unwrap();

    let first = besluiten.first().unwrap().to_owned();

    assert_eq!(
        first.id,
        Some(TkId::from("6b15c18f-fdc5-4ea4-bee2-a556e94d7682"))
    );
    assert_eq!(first.status.unwrap().as_str(), "Besluit");

    mock.assert_async().await;
}

#[tokio::test]
async fn get_vector_recursive_should_return_besluiten_when_request_is_successful()
 {
    let mut server = mockito::Server::new_async().await;

    let mut file = File::open("tests/responses/vec.besluit.1.json")
        .await
        .expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .expect("Could not read file");
    let resp = contents.replace("{{url}}", server.url().as_str());

    let mock = server
        .mock(
            "GET",
            "/Besluit?$filter=Verwijderd%20eq%20false%20and%20(BesluitSoort%\
             20eq%20%27Stemmen%20-%20aangenomen%27%20or%20BesluitSoort%20eq%\
             20%27Stemmen%20-%20verworpen%27)%20and%20GewijzigdOp%20ge%\
             202022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp%20asc%20&\
             $expand=Zaak($filter=Verwijderd%20eq%20false;\
             $expand=ZaakActor($filter=Relatie%20eq%20%27Indiener%27%20or%\
             20Relatie%20eq%20%27Medeindiener%27%20and%20Verwijderd%20eq%\
             20false),Document($filter=Verwijderd%20eq%20false)),\
             Stemming($filter=Verwijderd%20eq%20false),\
             Agendapunt($filter=Verwijderd%20eq%20false;\
             $expand=Activiteit($filter=Soort%20eq%20%27Stemmingen%27))",
        )
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body(resp)
        .create_async()
        .await;

    let mock2 = server
        .mock(
            "GET",
            "/Besluit?$filter=Verwijderd%20eq%20false%20and%20%28BesluitSoort%\
             20eq%20%27Stemmen%20-%20aangenomen%27%20or%20BesluitSoort%20eq%\
             20%27Stemmen%20-%20verworpen%27%29%20and%20GewijzigdOp%20ge%\
             202022-10-02T11%3A34%3A00.0-02%3A00&$orderby=GewijzigdOp%20asc%\
             20&$expand=Zaak%28%24filter%3DVerwijderd%20eq%20false%3B%\
             24expand%3DZaakActor%28%24filter%3DRelatie%20eq%20%27Indiener%27%\
             20or%20Relatie%20eq%20%27Medeindiener%27%20and%20Verwijderd%20eq%\
             20false%29%2CDocument%28%24filter%3DVerwijderd%20eq%20false%29%\
             29%2CStemming%28%24filter%3DVerwijderd%20eq%20false%29%\
             2CAgendapunt%28%24filter%3DVerwijderd%20eq%20false%3B%24expand%\
             3DActiviteit%28%24filter%3DSoort%20eq%20%27Stemmingen%27%29%29&\
             $skip=250",
        )
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/responses/vec.besluit.2.json")
        .create_async()
        .await;

    let client = Client::new_with_base_url(server.url().as_str()).unwrap();

    let result = client
        .get_vector_recursive::<Besluit>(
            "$filter=Verwijderd eq false and (BesluitSoort eq 'Stemmen - \
             aangenomen' or BesluitSoort eq 'Stemmen - verworpen') and \
             GewijzigdOp ge 2022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp \
             asc &$expand=Zaak($filter=Verwijderd eq \
             false;$expand=ZaakActor($filter=Relatie eq 'Indiener' or Relatie \
             eq 'Medeindiener' and Verwijderd eq \
             false),Document($filter=Verwijderd eq \
             false)),Stemming($filter=Verwijderd eq \
             false),Agendapunt($filter=Verwijderd eq \
             false;$expand=Activiteit($filter=Soort eq 'Stemmingen'))",
            Some(1),
        )
        .await;

    assert!(result.is_ok());

    let besluiten = result.unwrap();

    let first = besluiten.first().unwrap().to_owned();

    assert_eq!(
        first.id,
        Some(TkId::from("6b15c18f-fdc5-4ea4-bee2-a556e94d7682"))
    );
    assert_eq!(first.status.unwrap().as_str(), "Besluit");

    let last = besluiten.last().unwrap().to_owned();

    assert_eq!(
        last.id,
        Some(TkId::from("78926267-b11c-4e9a-9602-d35b581aeb60"))
    );
    assert_eq!(last.status.unwrap().as_str(), "Besluit");

    mock.assert_async().await;
    mock2.assert_async().await;
}

#[tokio::test]
async fn get_vector_with_response_should_return_besluiten_when_request_is_successful()
 {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/Besluit?$filter=Verwijderd%20eq%20false%20and%20(BesluitSoort%\
             20eq%20%27Stemmen%20-%20aangenomen%27%20or%20BesluitSoort%20eq%\
             20%27Stemmen%20-%20verworpen%27)%20and%20GewijzigdOp%20ge%\
             202022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp%20asc%20&\
             $expand=Zaak($filter=Verwijderd%20eq%20false;\
             $expand=ZaakActor($filter=Relatie%20eq%20%27Indiener%27%20or%\
             20Relatie%20eq%20%27Medeindiener%27%20and%20Verwijderd%20eq%\
             20false),Document($filter=Verwijderd%20eq%20false)),\
             Stemming($filter=Verwijderd%20eq%20false),\
             Agendapunt($filter=Verwijderd%20eq%20false;\
             $expand=Activiteit($filter=Soort%20eq%20%27Stemmingen%27))",
        )
        .with_status(200)
        .with_header("Content-Type", "application/json")
        .with_body_from_file("tests/responses/vec.besluit.1.json")
        .create_async()
        .await;

    let client = Client::new_with_base_url(server.url().as_str()).unwrap();
    let result = client
        .get_vector_with_response::<Besluit>(
            "$filter=Verwijderd eq false and (BesluitSoort eq 'Stemmen - \
             aangenomen' or BesluitSoort eq 'Stemmen - verworpen') and \
             GewijzigdOp ge 2022-10-02T11:34:00.0-02:00&$orderby=GewijzigdOp \
             asc &$expand=Zaak($filter=Verwijderd eq \
             false;$expand=ZaakActor($filter=Relatie eq 'Indiener' or Relatie \
             eq 'Medeindiener' and Verwijderd eq \
             false),Document($filter=Verwijderd eq \
             false)),Stemming($filter=Verwijderd eq \
             false),Agendapunt($filter=Verwijderd eq \
             false;$expand=Activiteit($filter=Soort eq 'Stemmingen'))",
        )
        .await;

    assert!(result.is_ok());

    let besluiten = result.unwrap();

    let first = besluiten.value.first().unwrap().to_owned();

    assert_eq!(
        first.id,
        Some(TkId::from("6b15c18f-fdc5-4ea4-bee2-a556e94d7682"))
    );
    assert_eq!(first.status.unwrap().as_str(), "Besluit");

    mock.assert_async().await;
}

#[tokio::test]
async fn get_resource_should_return_resourceresponse_when_request_is_successful()
 {
    let mut server = mockito::Server::new_async().await;

    let mock = server
        .mock(
            "GET",
            "/Document(2470914b-8978-4dc1-bc7e-c999e2671d21)/resource",
        )
        .with_status(200)
        .with_header("Content-Type", "application/pdf")
        .with_header(
            "Content-Disposition",
            "attachment; filename=2470914b-8978-4dc1-bc7e-c999e2671d21.pdf",
        )
        .with_body_from_file("tests/files/bestand.bin")
        .create_async()
        .await;

    let client = Client::new_with_base_url(server.url().as_str()).unwrap();

    let result = client
        .get_resource::<Document>(
            "2470914b-8978-4dc1-bc7e-c999e2671d21",
            "tests/tmp/",
        )
        .await;

    assert!(result.is_ok());

    let document = result.unwrap();

    assert_eq!(
        document.filename,
        "2470914b-8978-4dc1-bc7e-c999e2671d21.pdf"
    );
    assert_eq!(
        document.path,
        "tests/tmp/2470914b-8978-4dc1-bc7e-c999e2671d21.pdf"
    );

    _ = std::fs::remove_file(document.path);

    mock.assert_async().await;
}
