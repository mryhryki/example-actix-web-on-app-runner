use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct S3Object {
    pub key: String,
}

#[get("/listobjectsv2")]
async fn listobjectsv2(_path: web::Path<()>) -> impl Responder {
    println!("GET /listobjectsv2");
    let client = s3::Client::from_env();
    let req = client.list_objects_v2().bucket("mryhryki-temp");
    let resp = req.send().await;

    let mut contents: Vec<S3Object> = vec![];
    match resp {
        Ok(val) => match val.contents {
            Some(val) => {
                for object in val {
                    contents.push(S3Object {
                        key: object.key.unwrap_or(String::from("(ERROR)")),
                    })
                }
            }
            None => (),
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
    serde_json::to_string_pretty(&contents).unwrap()
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
