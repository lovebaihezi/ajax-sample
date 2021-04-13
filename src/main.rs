use actix_files as fs;
use actix_web::{App, HttpServer};
use router::post::{end, query, start, username};

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(start)
            .service(end)
            .service(username)
            .service(query)
            .service(fs::Files::new("/", "../static/").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
