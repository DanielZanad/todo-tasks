use actix_web::error;

use crate::{
    app::{entities::user::User, repositories::user_repository::UserRepository},
    infra::db::configuration::get_configuration,
};

pub struct SqlxRepository {}

impl UserRepository for SqlxRepository {
    fn register<'a>(
        &'a self,
        user: User,
        file_key: String,
        mime_type: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let db_conn = get_configuration().await.unwrap();

            let mut transaction = db_conn
                .begin()
                .await
                .map_err(|_| {
                    error::ErrorInternalServerError("Failed to start database transaction.")
                })
                .map_err(|e| eprint!("Failed to start a transaction: {}", e))
                .unwrap();

            let new_user = sqlx::query!(
            "INSERT INTO users (email, username, password_hash) VALUES ($1, $2, $3) RETURNING id",
            user.email(),
            user.username(),
            user.password(),
        )
        .fetch_one(&mut *transaction)
        .await
        .map_err(|e| {
            eprintln!("Failed to create user: {}", e);
            error::ErrorInternalServerError("Failed to register user.")
        }).map_err(|e| {
            eprintln!("Failed to create an user: {}", e)
        }).unwrap();

            let user_id = new_user.id;

            sqlx::query(
                r#"
        INSERT INTO avatars (user_id, file_key, mime_type)
        VALUES ($1, $2, $3)
        "#,
            )
            .bind(user_id)
            .bind(file_key)
            .bind(mime_type)
            .execute(&mut *transaction)
            .await
            .map_err(|e| {
                eprintln!("Failed to create avatar: {}", e);
                error::ErrorInternalServerError("Failed to save user avatar.")
            })
            .map_err(|e| eprintln!("Failed to create an avatar: {}", e))
            .unwrap();

            transaction
                .commit()
                .await
                .map_err(|_| {
                    error::ErrorInternalServerError("Failed to commit database transaction.")
                })
                .map_err(|e| eprintln!("Failed to commit the transaction: {}", e))
                .unwrap()
        })

        // --- Start a database transaction ---
    }
}
