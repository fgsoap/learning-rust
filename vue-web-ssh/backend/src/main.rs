use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};

mod ssh;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .service(web::resource("/ssh").route(web::post().to(ssh::exec)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
