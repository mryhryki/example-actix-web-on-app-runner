use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use auth::default_provider;
use s3::{Config, Region};

#[get("/")]
async fn index(_path: web::Path<()>) -> impl Responder {
    String::from("Hello, Actix Web!")
}

#[get("/listbuckets")]
async fn list_buckets(_path: web::Path<()>) -> impl Responder {
    let config = Config::builder()
        .region(Region::new("us-east-1"))
        .credentials_provider(default_provider())
        .build();
    let resp = s3::Client::from_conf(config).list_buckets().send().await;

    match resp {
        Ok(val) => {
            let list: Vec<String> = val
                .buckets
                .unwrap_or(vec![])
                .iter()
                .map(|bucket| {
                    String::from(bucket.name.as_ref().unwrap_or(&String::from("(ERROR)")))
                })
                .collect();
            HttpResponse::Ok().json(&list)
        }
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(list_buckets)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
