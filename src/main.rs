mod lib;
mod tests;

use actix_web::middleware::Logger;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use serde::Deserialize;
use std::sync::Once;
use actix_web::rt::Runtime;
use actix_files::Files;
use actix_cors::Cors;
use std::mem::drop;

extern crate log;

use log::{debug, error, log_enabled, info, Level};

use exitfailure::ExitFailure;
use std::thread;
use rust_bert::pipelines::common::ModelType;
use tch::Device;


#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[derive(Deserialize)]
struct Info {
    context: String,
    minlength: i64,
    model: ModelType,
}


#[get("/api/health")]
async fn api_health_handler() -> HttpResponse {
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: "Health Check".to_string(),
    };
    HttpResponse::Ok().json(response_json)
}


#[post("/api/summary")]
async fn api_summary_handler(info: web::Json<Info>) -> impl Responder {
    let summarization_model = lib::init_summarization_model(info.model, info.minlength);
    info!("init model success");
    let this_device = Device::cuda_if_available();
    match this_device {
        Device::Cuda(_) => info!("Using GPU"),
        Device::Cpu => info!("Using CPU"),
        _ => {}
    }


    let mut input = [String::new(); 1];
    input[0] = info.context.to_owned();

    let _output = summarization_model.summarize(&input);
    let mut result = String::from(_output.join(" "));
    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: result.to_string(),
    };

    info!("Response message: {}", response_json.message);

    HttpResponse::Ok().json(response_json)
}


#[actix_web::main]
async fn main() -> Result<(), ExitFailure> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Server started successfully");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin() // Allow requests from any origin
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .max_age(3600);


        App::new()
            // .wrap(cors) // Add the CORS middleware to the app
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