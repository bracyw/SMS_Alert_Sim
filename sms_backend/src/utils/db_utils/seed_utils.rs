use sea_orm::*;
use ::entity::user::Entity as User;
use ::entity::user::ActiveModel as UserModel;

pub async fn seed_if_empty(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Check if the User table is empty
    let user_count = User::find().count(db).await?;

    if user_count == 0 {
        println!("Seeding database: Adding default user...");

        // Create a new user with name and password set to "123"
        let new_user = UserModel {
            name: Set("123".to_owned()),
            password: Set("123".to_owned()),
            ..Default::default()
        };

        // Insert the new user into the database
        new_user.insert(db).await?;
        println!("Default user created.");
    } else {
        println!("Database already seeded.");
    }

    Ok(())
}
