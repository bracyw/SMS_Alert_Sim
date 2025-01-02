use sea_orm::{Database, DatabaseConnection, EntityTrait, Set};
use dotenvy::dotenv;
use std::env;
use entity::{user, alert};

// AI GENERATED CODE

/// Connect to the PostgreSQL database using SeaORM.
pub async fn connect_to_database() -> DatabaseConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

/// Reset the database by deleting all rows from relevant tables.
pub async fn reset_database(connection: &DatabaseConnection) {
    // Delete all data from the `Alert` table
    alert::Entity::delete_many()
        .exec(connection)
        .await
        .expect("Failed to delete alerts");

    // Delete all data from the `User` table
    user::Entity::delete_many()
        .exec(connection)
        .await
        .expect("Failed to delete users");
}

/// Seed the database with initial test data.
pub async fn seed_database(connection: &DatabaseConnection) {
    // Insert test users
    let users = vec![
        (1,"Alice", "password1"),
        (2, "Bob", "password2"),
    ];

    for (id, name, password) in users {
        let new_user = user::ActiveModel {
            id: Set(id),
            name: Set(name.to_owned()),
            password: Set(password.to_owned()),
            ..Default::default()
        };
        user::Entity::insert(new_user)
            .exec(connection)
            .await
            .expect("Failed to insert test user");
    }

    // Insert test alerts
    let alerts = vec![
        (1, 100, Some("First Alert")), // Assuming user with ID 1 exists
        (2, 200, Some("Second Alert")), // Assuming user with ID 2 exists
    ];

    for (user_id, send_amount_requested, message) in alerts {
        let new_alert = alert::ActiveModel {
            user_id: Set(user_id),
            send_amount_requested: Set(send_amount_requested),
            message: Set(message.map(|m| m.to_owned())),
            messages_sent: Set(0),
            messages_failed: Set(0),
            total_time_to_send: Set(0),
            ..Default::default()
        };
        alert::Entity::insert(new_alert)
            .exec(connection)
            .await
            .expect("Failed to insert test alert");
    }
}

/// Utility function to reset and seed the database.
pub async fn reset_and_seed_database(connection: &DatabaseConnection) {
    reset_database(connection).await;
    seed_database(connection).await;
}
