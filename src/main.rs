mod lib;
mod tests;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use serde::Deserialize;
use std::sync::Once;
use actix_web::rt::Runtime;
use actix_files::Files;


use exitfailure::ExitFailure;
use std::thread;



#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[derive(Deserialize)]
struct Info {
    context: String,
}


#[get("/api/health")]
async fn api_health_handler() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}



#[get("/api/summary")]
async fn api_summary_handler(info: web::Query<Info>) -> impl Responder {
    let summarization_model = lib::init_summarization_model();

    let mut input = [String::new(); 1];
    input[0] = info.context.to_owned();

    let _output = summarization_model.summarize(&input);
    let mut result = String::from(_output.join(" "));
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: result.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}







#[actix_web::main]
async fn main() -> Result<(), ExitFailure> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();


    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(api_health_handler)
            .service(api_summary_handler)
            .service(Files::new("/", "./dist").index_file("index.html"))

            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;
    Ok(())
}