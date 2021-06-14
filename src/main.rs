use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index(web::Path(()): web::Path<()>) -> impl Responder {
    format!("Hello Actice-Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let target = "0.0.0.0:8080";
    println!("Listen: {}", &target);
    HttpServer::new(|| App::new().service(index))
        .bind(target)?
        .run()
        .await
}
