use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use tera::{Tera, Context};

mod config;
mod csv_reader;
mod lib;
mod http;


async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    let file_path = config::load_env_vars("CRYPTO");
    println!("THE PATH TO CSV {}", file_path);

    let crypto_data = match csv_reader::read_csv_file(&file_path) {
        Ok(data) => data,
        Err(error) => panic!("Could not read from csv{:?}", error),
    };

    println!("Crypto Data: {:?}", crypto_data);
    
    data.insert("crypto_data", &crypto_data);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on http://127.0.0.1:9000");

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await
}