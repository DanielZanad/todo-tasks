use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    dev::Server,
    get, http,
    web::{self, Data},
};
use sqlx::{PgPool, Pool, Postgres};

use crate::routes::register_user::register_user;

pub struct AppState {
    pub conn: PgPool,
}

impl AppState {
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let app_data = web::Data::new(AppState::new(db_pool));
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(health_check)
            .service(register_user)
            .app_data(app_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
