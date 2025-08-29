use std::net::TcpListener;
use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, dev::Server, get, http, middleware::from_fn, web,
};
use sqlx::PgPool;

use crate::{
    app::use_cases::{
        create_user_session_use_case::CreateUserSessionUseCase,
        get_signed_url_use_case::GetSignedUrlUseCase,
        get_user_profile_use_case::GetUserProfileUseCase,
        register_user_use_case::RegisterUserUseCase,
    },
    infra::{
        db::sqlx_repository::SqlxRepository,
        http::{
            create_user_session_controller::create_user_session_controller,
            get_user_profile_controller::get_user_profile_controller,
            register_user_controller::register_user_controller,
        },
        middlewares::check_request_jwt::check_request_jwt,
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
    let get_signed_url_use_case = web::Data::new(GetSignedUrlUseCase::new());
    let create_user_session_use_case =
        web::Data::new(CreateUserSessionUseCase::new(Arc::new(SqlxRepository {})));
    let get_user_profile_use_case =
        web::Data::new(GetUserProfileUseCase::new(Arc::new(SqlxRepository {})));

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
            .service(register_user_controller)
            .service(create_user_session_controller)
            .service(
                web::scope("/users")
                    .wrap(from_fn(check_request_jwt))
                    .service(get_user_profile_controller),
            )
            .app_data(register_user_use_case.clone())
            .app_data(create_user_session_use_case.clone())
            .app_data(get_signed_url_use_case.clone())
            .app_data(get_user_profile_use_case.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
