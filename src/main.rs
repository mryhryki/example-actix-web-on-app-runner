use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct S3Object {
    pub key: String,
}

#[get("/listbuckets")]
async fn listobjectsv2(_path: web::Path<()>) -> impl Responder {
    println!("GET /listbuckets");
    let resp = s3::Client::from_env()
        .list_buckets()
        .send()
        .await;

    let mut list: Vec<String> = vec![];
    match resp {
        Ok(val) => match val.buckets {
            Some(buckets) => {
                for bucket in buckets {
                    list.push(bucket.name.unwrap_or(String::from("(ERROR)")))
                }
            }
            None => (),
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
    serde_json::to_string_pretty(&list).unwrap()
}

#[get("/")]
async fn index(_path: web::Path<()>) -> impl Responder {
    println!("GET /");
    String::from("Hello, Actix Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let target = "0.0.0.0:8080";
    println!("Listen: {}", &target);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(listobjectsv2)
    })
    .bind(target)?
    .run()
    .await
}
