use zero2prod::main;

#[actix_rt::test]
async fn health_check_works() {
    spwan_app().await.expect("Failed to spawn our app");

    let client = reqwest::Client::new();

    let response = client.get("http://127.0.0.1:7777/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spwan_app() -> std::io::Result<()> {
    todo!()
}
