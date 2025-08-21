use std::net::TcpListener;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get, http, web};
use sqlx::PgPool;

use crate::{
    app::use_cases::register_user_use_case::RegisterUserUseCase,
    infra::{
        db::sqlx_repository::SqlxRepository, http::register_user_controller::register_user_route,
    },
};

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

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let register_user_use_case =
        web::Data::new(RegisterUserUseCase::new(Arc::new(SqlxRepository {})));

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
            .service(register_user_route)
            .app_data(register_user_use_case.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
