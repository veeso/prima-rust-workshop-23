//! # Web server

mod auth_api;
mod graphql_api;
mod health_check;
mod session;

use session::SessionClient;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{dev::Server, web::Data, App as ActixApp, HttpServer};
use std::net::TcpListener;

pub struct WebServer {
    server: Server,
}

struct WebserverData {
    pub store_client_url: String,
}

impl WebServer {
    /// Initialize web server
    pub async fn init(protobuf_url: &str, web_port: u16) -> anyhow::Result<Self> {
        debug!("webserver initialized");
        debug!("protobuf url: {protobuf_url}");
        debug!("web port: {web_port}");

        let listener = TcpListener::bind(&format!("0.0.0.0:{web_port}"))?;
        let secret_key = Key::generate();

        let server = {
            let protobuf_url = protobuf_url.to_string();
            HttpServer::new(move || {
                let web_data = Data::new(WebserverData {
                    store_client_url: protobuf_url.to_string(),
                });
                ActixApp::new()
                    .service(graphql_api::service_factory(&protobuf_url))
                    .service(health_check::check_action)
                    .service(auth_api::sign_in)
                    .service(auth_api::sign_up)
                    .service(auth_api::auth)
                    .app_data(web_data)
                    .wrap(SessionMiddleware::new(
                        CookieSessionStore::default(),
                        secret_key.clone(),
                    ))
            })
            .listen(listener)?
            .run()
        };

        info!("web server initialized");
        Ok(Self { server })
    }

    /// Run web server
    pub async fn run(self) -> anyhow::Result<()> {
        info!("running web server");
        self.server.await?;
        info!("web server stopped");
        Ok(())
    }
}
