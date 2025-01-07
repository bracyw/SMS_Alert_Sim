use axum::Router;
use dotenvy;
use sea_orm::{Database, DatabaseConnection};
use tokio::net::TcpListener;
use tracing::info;
use std::sync::Arc;
use structures::progress_monitor::ProgressMonitor;
use tower_http::cors::{CorsLayer, Any};
pub mod structures;
pub mod handlers;
pub mod utils;
pub mod models;
pub mod routes;
pub mod services;
use utils::{db_utils, sms_utils::sender_utils::create_sender_service};

/// Main function to start the sms simulator backend, by intializing the listening to the server address, 
/// setting up the database connection, and creating the routes for the application using axum.
/// As well as initalizing structures used throughout the lifetime of the application (`SenderService`, `ProgressMontior`).
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Unable to access .env file");

    // Initialize tracing (for debugging and logging info)
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();


    // Prior to anything else, ensure we have a database url in the .env file
    // also obtain the server address from the .env file or use default value
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:4201".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in the env variables");

    // Connect to the database (should be running in docker)
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();

    // create TCP listener for our server address
    let listener = TcpListener::bind(server_address).await.expect("Could not create tcp listener");

    info!("listening on {}", listener.local_addr().unwrap());

    // WARNING: In production origin should be restricted to the frontend server address
    // for ease of development we will allow any origin and any headers
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);

    // Create the sender service and progress monitor to be used throughout the lifetime of the application
    let pm = Arc::new(ProgressMonitor::new());
    let sender_service = Arc::new(create_sender_service(100, 1, 1.0, 1.0, pm.clone()).await.expect("Error creating sender service"));


    // Create a basic user for the database with password and username (this is used to mock a user for the alert data)
    db_utils::seed_utils::seed_if_empty(&db).await.expect("Error seeding database");
    
    // Create the application routes sea routes folder for more info on all routes
    let app: Router = Router::new()
        .nest("/system", routes::system_routes::system_routes(sender_service.clone())) // System-level sender service controls
        .nest("/alert", routes::alert_routes::alert_routes(sender_service.clone())) // User-facing alert actions
        .nest("/progress-monitor", routes::progress_monitor_routes::progress_monitor_routes(pm.clone())) // System monitoring
        .nest("/historical", routes::historical_routes::historical_routes()) // Historical data
        .layer(cors) // Enable CORS for cross-origin requests (allows for fetch in the frontend)
        .with_state(db); // Provide shared database state

    axum::serve(listener, app).await.expect("Error serving application");
}
