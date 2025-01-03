use learn_with_edu_chain::create_app;  // Paket adını düzelttik
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let blockchain_url = env::var("BLOCKCHAIN_URL")
        .expect("BLOCKCHAIN_URL must be set");
    let contract_address = env::var("CONTRACT_ADDRESS")
        .expect("CONTRACT_ADDRESS must be set");
    let private_key = env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set");

    let server = create_app(
        &database_url,
        &blockchain_url,
        &contract_address,
        &private_key,
    ).await?;

    println!("Server running at http://127.0.0.1:8080");

    server.await
}