pub mod api;
pub mod blockchain;
pub mod models;
pub mod services;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;

pub struct AppState {
    pub course_service: Arc<services::course_service::CourseService>,
}

pub async fn create_app(
    database_url: &str,
    blockchain_url: &str,
    contract_address: &str,
    private_key: &str,
) -> std::io::Result<actix_web::dev::Server> {
    let blockchain_service = blockchain::BlockchainService::new(
        blockchain_url,
        contract_address.parse().expect("Invalid contract address"),
        private_key,
    ).await.expect("Failed to initialize blockchain service");

    let pool = sqlx::PgPool::connect(database_url)
        .await
        .expect("Failed to connect to database");

    let course_service = Arc::new(services::course_service::CourseService::new(
        blockchain_service,
        pool,
    ));

    let app_state = web::Data::new(AppState {
        course_service: course_service.clone(),
    });

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("/api/v1")
                    .configure(api::courses::config)
                    .configure(api::certificates::config)
            )
    })
        .bind("127.0.0.1:8080")?
        .run();

    Ok(server)
}