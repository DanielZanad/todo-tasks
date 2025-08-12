use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::env::get_env_var;

pub async fn get_configuration() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = get_env_var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
