use sea_orm::{EntityTrait, PaginatorTrait, Set, ActiveModelTrait};
use ::entity::user::Entity as User;
use serial_test::serial;
use sms_backend::utils::db_utils::seed_utils::seed_if_empty;
mod test_utils;
use test_utils::{connect_to_database, reset_database};

#[tokio::test]
#[serial]
async fn test_seed_if_empty_when_empty() {
    // Connect to an in-memory SQLite database for testing
    let db = connect_to_database().await;

    // Run migrations to set up the User table
    reset_database(&db).await;

    // Call the seed_if_empty function
    seed_if_empty(&db).await.unwrap();

    // Verify that a user was created
    let user_count = User::find().count(&db).await.unwrap();
    assert_eq!(user_count, 1, "Expected 1 user to be seeded");
}

#[tokio::test]
#[serial]
async fn test_seed_if_empty_when_not_empty() {
    // Connect to an in-memory SQLite database for testing
    let db = connect_to_database().await;

    // Run migrations to set up the User table
    reset_database(&db).await;

    // Seed the database manually
    let user_model = ::entity::user::ActiveModel {
        name: Set("Alice".to_owned()),
        password: Set("password123".to_owned()),
        ..Default::default()
    };
    user_model.insert(&db).await.unwrap();

    // Call the seed_if_empty function
    seed_if_empty(&db).await.unwrap();

    // Verify that no additional users were added
    let user_count = User::find().count(&db).await.unwrap();
    assert_eq!(user_count, 1, "Expected 1 user (no new users seeded)");
}
