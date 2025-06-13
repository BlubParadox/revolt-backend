use std::{env, net::{Ipv4Addr, SocketAddr}};

use axum::Router;
use tokio::net::TcpListener;
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_scalar::{Scalar, Servable as ScalarServable};

mod api;
pub mod requests;
pub mod website_embed;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Configure logging and environment
    revolt_config::configure!(proxy);
	
	tracing_subscriber::fmt::init();

    // Configure API schema
    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        paths(
            api::root,
            api::proxy,
            api::embed
        ),
        components(
            schemas(
                api::RootResponse,
                revolt_result::Error,
                revolt_result::ErrorType,
                revolt_models::v0::ImageSize,
                revolt_models::v0::Image,
                revolt_models::v0::Video,
                revolt_models::v0::TwitchType,
                revolt_models::v0::LightspeedType,
                revolt_models::v0::BandcampType,
                revolt_models::v0::Special,
                revolt_models::v0::WebsiteMetadata,
                revolt_models::v0::Text,
                revolt_models::v0::Embed
            )
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
                )
            }
        }
    }

    // Configure Axum and router
    let app = Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .nest("/", api::router().await);

    // Use PORT from env or default to 14705
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(14705);
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

    tracing::info!("Listening on 0.0.0.0:{port}");
    tracing::info!("Play around with the API: http://localhost:{port}/scalar");

    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, app.into_make_service()).await
}
