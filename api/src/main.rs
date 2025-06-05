use axum::{routing::get, Json, Router};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct SwiftLintRelease {
	pub tag_name: String,
}

#[utoipa::path(
	get,
	path = "/get_tag",
	responses(
        (
            status = 200,
            description = "Latest release tag from SwiftLint GitHub",
            body = SwiftLintRelease
        ),
        (
            status = 502,
            description = "Failed to contact GitHub"
        )
	)
)]

async fn get_tag() -> Result<Json<SwiftLintRelease>, (axum::http::StatusCode, String)> {
	let url = "https://api.github.com/repos/realm/SwiftLint/releases/latest";

	let client = Client::new();
	let response = client
		.get(url)
		.header("User-Agent", "axum-swagger-example")
		.send()
		.await
		.map_err(|e| (axum::http::StatusCode::BAD_GATEWAY, e.to_string()))?;

	let release = response
		.json::<SwiftLintRelease>()
		.await
		.map_err(|e| (axum::http::StatusCode::BAD_GATEWAY, e.to_string()))?;

	Ok(Json(release))
}

#[derive(OpenApi)]
#[openapi(paths(get_tag), components(schemas(SwiftLintRelease)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
	let app = Router::new()
		.route("/get_tag", get(get_tag))
		.merge(
			SwaggerUi::new("/swagger-ui")
				.url(
					"/api-doc/openapi.json", 
					 ApiDoc::openapi()
				)
		);

	let address = SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = TcpListener::bind(address).await.unwrap();

	println!("🚀 http://{}/get_tag", address);
	println!("📘 Swagger UI: http://{}/swagger-ui", address);

	axum::serve(listener, app).await.unwrap();
}