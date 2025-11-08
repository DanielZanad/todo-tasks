use actix_web::error;

use crate::{
    app::{
        entities::{task_status::TaskStatus, user::User},
        repositories::{
            task_repository::TaskRepository,
            user_repository::{UserProfile, UserRepository},
        },
    },
    infra::db::{
        configuration::get_configuration,
        mappers::sqlx_task_mapper::{chrono_to_primitive, to_domain},
    },
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
            let task_date = chrono_to_primitive(*task.task_date());

            println!("Primitive Date time {:?}", task_date);

            sqlx::query!(
                "INSERT INTO tasks (user_id, content, task_date ,tasks_status) VALUES ($1, $2, $3, $4)",
                uuid::Uuid::parse_str(&task.user_id()).unwrap(),
                task.content(),
                task_date,
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

    fn list_all<'a>(
        &'a self,
        user_id: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = Vec<crate::app::entities::task::Task>> + Send + 'a>>
    {
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

            use uuid::Uuid;

            // Convert user_id from String to Uuid
            let user_uuid = match Uuid::parse_str(&user_id) {
                Ok(uuid) => uuid,
                Err(e) => {
                    eprintln!("Invalid UUID: {}", e);
                    transaction.rollback().await.ok();
                    return Vec::new();
                }
            };

            // Explicitly select columns and map tasks_status to an integer
            let rows = sqlx::query!(
                r#"
                SELECT id, user_id, content, task_date, created_at ,tasks_status as "tasks_status: TaskStatus"
                FROM tasks
                WHERE user_id = $1
                "#,
                user_uuid
            )
            .fetch_all(&mut *transaction)
            .await
            .map_err(|e| {
                eprintln!("Failed to fetch tasks: {}", e);
                error::ErrorInternalServerError("Failed to list tasks.")
            })
            .map_err(|e| eprintln!("Failed to fetch tasks: {}", e))
            .unwrap_or_default();

            transaction
                .commit()
                .await
                .map_err(|_| {
                    error::ErrorInternalServerError("Failed to commit database transaction.")
                })
                .map_err(|e| eprintln!("Failed to commit the transaction: {}", e))
                .unwrap();

            rows.into_iter()
                .map(|row| {
                    // let task_status;
                    // match row.tasks_status {
                    //     TaskStatus::ToStart => task_status = TaskStatus::ToStart,
                    //     TaskStatus::Started => task_status = TaskStatus::Started,
                    //     TaskStatus::Completed => task_status = TaskStatus::Completed,
                    // }

                    let created_at_utc = to_domain(row.created_at.unwrap());
                    let task_date_utc = to_domain(row.task_date);

                    // Convert LocalResult<chrono::DateTime<Utc>> to chrono::DateTime<Local>
                    let created_at = match created_at_utc {
                        chrono::LocalResult::Single(dt_utc) => dt_utc.with_timezone(&chrono::Utc),
                        _ => chrono::Utc::now(), // fallback or handle error as needed
                    };
                    println!("created_at: {:?}", created_at);
                    let task_date = match task_date_utc {
                        chrono::LocalResult::Single(dt_utc) => dt_utc.with_timezone(&chrono::Utc),
                        _ => chrono::Utc::now(), // fallback or handle error as needed
                    };

                    println!("Tasks_date: {:?}", task_date);

                    crate::app::entities::task::Task::new_with_id(
                        row.id.to_string(),
                        row.user_id.to_string(),
                        row.content,
                        row.tasks_status,
                        task_date,
                        created_at,
                    )
                })
                .collect()
        })
    }

    fn find_by_id<'a>(
        &'a self,
        task_id: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = Option<crate::app::entities::task::Task>> + Send + 'a>>
    {
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

            use uuid::Uuid;

            let task_uuid = match Uuid::parse_str(&task_id) {
                Ok(uuid) => uuid,
                Err(e) => {
                    eprintln!("Invalid UUID: {}", e);
                    transaction.rollback().await.ok();
                    return None;
                }
            };
            let row = sqlx::query!(
                r#"
                SELECT id, user_id, content, task_date, created_at, tasks_status as "tasks_status: TaskStatus"
                FROM tasks
                WHERE id = $1
                "#,
                task_uuid
            )
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|e| {
                eprintln!("Failed to fetch task by ID: {}", e);
                error::ErrorInternalServerError("Failed to find task by ID.")
            })
            .map_err(|e| eprintln!("Failed to fetch task by ID: {}", e))
            .unwrap_or_default();

            transaction
                .commit()
                .await
                .map_err(|_| {
                    error::ErrorInternalServerError("Failed to commit database transaction.")
                })
                .map_err(|e| eprintln!("Failed to commit the transaction: {}", e))
                .unwrap();

            row.map(|row| {
                let created_at_utc = to_domain(row.created_at.unwrap());
                let task_date_utc = to_domain(row.task_date);

                let created_at = match created_at_utc {
                    chrono::LocalResult::Single(dt_utc) => dt_utc.with_timezone(&chrono::Utc),
                    _ => chrono::Utc::now(),
                };
                let task_date = match task_date_utc {
                    chrono::LocalResult::Single(dt_utc) => dt_utc.with_timezone(&chrono::Utc),
                    _ => chrono::Utc::now(),
                };

                crate::app::entities::task::Task::new_with_id(
                    row.id.to_string(),
                    row.user_id.to_string(),
                    row.content,
                    row.tasks_status,
                    task_date,
                    created_at,
                )
            })
        })
    }

    fn update_status<'a>(
        &'a self,
        user_id: String,
        task_id: String,
        status: TaskStatus,
    ) -> std::pin::Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let db_conn = get_configuration().await.unwrap();
            sqlx::query!(
                "UPDATE tasks SET tasks_status = $1 WHERE id = $2 AND user_id = $3",
                status as _,
                uuid::Uuid::parse_str(&task_id).unwrap(),
                uuid::Uuid::parse_str(&user_id).unwrap()
            )
            .execute(&db_conn)
            .await
            .map_err(|e| {
                eprintln!("Failed to update task status: {}", e);
                error::ErrorInternalServerError("Failed to update task status.")
            })
            .unwrap();
        })
    }
}
