use actix_web::error;

use crate::{
    app::{
        entities::user::User,
        repositories::{
            task_repository::TaskRepository,
            user_repository::{UserProfile, UserRepository},
        },
    },
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
    }

    fn get_user_by_email<'a>(
        &'a self,
        email: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = Option<User>> + Send + 'a>> {
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

            let user = sqlx::query!("SELECT * FROM users WHERE email = $1", email)
                .fetch_one(&mut *transaction)
                .await
                .map_err(|e| {
                    eprintln!("Failed to an user by email: {}", e);
                    error::ErrorInternalServerError("Failed to get an user by email.")
                })
                .ok()?;

            transaction
                .commit()
                .await
                .map_err(|_| {
                    error::ErrorInternalServerError("Failed to commit database transaction.")
                })
                .map_err(|e| eprintln!("Failed to commit the transaction: {}", e))
                .unwrap();

            let user = User::new_with_id(
                user.id.to_string(),
                user.email,
                user.username,
                user.password_hash,
            );
            Some(user)
        })
    }

    fn get_user_profile<'a>(
        &'a self,
        user_id: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = Option<UserProfile>> + Send>> {
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

            let user_profile = sqlx::query!(
                "SELECT users.email, users.username, avatars.file_key FROM users INNER JOIN avatars ON $1 = avatars.user_id",
                uuid::Uuid::parse_str(&user_id).unwrap()
            )
            .fetch_one(&mut *transaction)
            .await
            .map_err(|e| {
                eprintln!("Failed to an user by email: {}", e);
                error::ErrorInternalServerError("Failed to get an user by email.")
            })
            .ok();

            match user_profile {
                Some(user_profile) => {
                    transaction
                        .commit()
                        .await
                        .map_err(|_| {
                            error::ErrorInternalServerError(
                                "Failed to commit database transaction.",
                            )
                        })
                        .map_err(|e| eprintln!("Failed to commit the transaction: {}", e))
                        .unwrap();

                    let user =
                        User::new_without_password(user_profile.email, user_profile.username);
                    let user_profile = UserProfile {
                        user,
                        avatar_url: user_profile.file_key,
                    };
                    Some(user_profile)
                }
                None => None,
            }
        })
    }
}

impl TaskRepository for SqlxRepository {
    fn save<'a>(
        &'a self,
        task: crate::app::entities::task::Task,
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

            let user = sqlx::query!(
                "SELECT * FROM users WHERE id = $1",
                uuid::Uuid::parse_str(&task.user_id()).unwrap()
            );

            if (user.fetch_one(&mut *transaction).await).is_err() {
                transaction
                    .rollback()
                    .await
                    .map_err(|_| {
                        error::ErrorInternalServerError("Failed to rollback database transaction.")
                    })
                    .map_err(|e| eprintln!("Failed to rollback the transaction: {}", e))
                    .unwrap();
                return;
            }

            let task_status = task.status();

            sqlx::query!(
                "INSERT INTO tasks (user_id, content, tasks_status) VALUES ($1, $2, $3)",
                uuid::Uuid::parse_str(&task.user_id()).unwrap(),
                task.content(),
                task_status as _
            )
            .execute(&mut *transaction)
            .await
            .map_err(|e| {
                eprintln!("Failed to create task: {}", e);
                error::ErrorInternalServerError("Failed to save task.")
            })
            .map_err(|e| eprintln!("Failed to create a task: {}", e))
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
    }
}
