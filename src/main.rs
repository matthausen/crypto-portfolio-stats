use actix_session::{Session};
use actix_web::http::{StatusCode};
use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Result};

mod config;
mod csv_reader;


#[get("/")]
async fn hello(_session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    let file_path = config::load_env_vars("CRYPTO");
    
    if let Err(e) = csv_reader::read_csv_file(&*file_path) {
        eprintln!("{}", e);
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/index.html")))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u32 = 9000;
    println!("Server started on port: {0}", port);
    HttpServer::new(|| {
        App::new()
        .wrap(middleware::Logger::default())
        .service(hello)
    })
    .bind("0.0.0.0:9000")?
    .run()
    .await
}
