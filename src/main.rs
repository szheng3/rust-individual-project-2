// use clap::Parser;
//
// mod lib;
// mod tests;
//
//
// #[derive(Parser)]
// #[clap(
// version = "1.0",
// author = "Shuai Zheng",
// about = "Command-line interface for Summarization"
// )]
// struct Cli {
//     #[clap(subcommand)]
//     command: Option<Commands>,
// }
//
// #[derive(Parser)]
// enum Commands {
//     #[clap(version = "1.0", author = "Shuai Zheng")]
//     Text {
//         #[clap(short, long)]
//         input: String,
//     },
// }
//
// fn main() {
//     let args = Cli::parse();
//     match args.command {
//         Some(Commands::Text { input }) => {
//             let summarization_model = lib::init_summarization_model();
//
//             let mut inputs = [String::new(); 1];
//             inputs[0] = input.to_owned();
//
//             let _output = summarization_model.summarize(&inputs);
//             let mut result = String::from(_output.join(" "));
//             println!("Below is the summary:");
//
//             println!("{:#?}", result);
//         }
//         None => {
//             println!("No command given");
//         }
//     }
// }
mod lib;
mod tests;
use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use serde::Serialize;
use serde::Deserialize;
use std::sync::Once;
use actix_web::rt::Runtime;


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

#[get("/")]
async fn api_health_handler2() -> HttpResponse {
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

    // let mut rt = Runtime::new().unwrap();
    // rt.block_on(async {
    //     init_summarization_model();
    //     drop(rt); // this line causes the error
    // });



    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(api_health_handler)
            .service(api_health_handler2)
            .service(api_summary_handler)

            .wrap(Logger::default())
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await?;
    Ok(())
}