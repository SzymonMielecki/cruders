mod handler;
mod model;
mod route;
mod test_helper;
use crate::route::{create_router, join_router_db};
mod db;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let db = db::init_users_db().await?;

    let app = join_router_db(create_router().layer(cors), db);
    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod unit_tests;
