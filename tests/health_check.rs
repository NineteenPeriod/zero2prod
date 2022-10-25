use std::net::TcpListener;
use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    let addess = spawn_app();

    let client = reqwest::Client::new();

    let app_address = format!("{}/health_check", &addess);

    let response = client
        .get(app_address)
        .send()
        .await
        .expect("Fallo al ejecutar la peticion.");


    assert!(response.status().is_success(), "La peticion  no fue exitosa.");
    assert_eq!(Some(0), response.content_length(), "La longitud del contenido no fue 0.");
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Error al bindear un puerto aleatorio");

    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Fallo al spawnear la app.");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}