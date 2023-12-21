use std::net::SocketAddr;
use tracing::info;

mod app;
mod logger;
mod models;
mod settings;

// There are a couple approaches to take when implementing E2E tests. This
// approach adds tests on /src/tests, this way tests can reference modules
// inside the src folder. Another approach would be to have the tests in a
// /tests folder on the root of the project, to do this and be able to import
// modules from the src folder, modules need to be exported as a lib.
// #[cfg(test)]
// mod tests;

// use errors::Error;
use settings::SETTINGS;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    info!("Server listening on {}", &address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app)
        .await
        .expect("Failed to start the server");
    // axum::Server::bind(&address)
    //     .serve(app.into_make_service())
    //     .await
    //     .expect("Failed to start server");
}
