use std::error::Error;

use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::Result;
use actix_web::http::header;
use askama::Template;
use serde::Serialize;
use clap::Parser;

#[derive(Template, Serialize)]
#[template(path = "index.html")]
struct Index {
    ip: String,
    user_agent: String,
    remote_addr: String
}

#[derive(Parser)]
struct Args {
    /// Listening addr
    #[clap(short, long, default_value = "0.0.0.0:8080")]
    addr: String
}

impl Index {
    fn from_request(req: HttpRequest) -> Index {
        let ip = if let Some(addr) = req.peer_addr() {
            addr.ip().to_string()
        } else {
            "unknown".to_string()
        };
        let user_agent = {
            let map = req.headers();
            map.get(header::USER_AGENT)
               .map_or("unknown".to_string(), 
                        |h| h.to_str().unwrap_or("unknown").to_string())
        };
        let remote_addr = req.connection_info().remote_addr().unwrap_or("unknown").to_string();

        Index {
            ip, 
            user_agent,
            remote_addr
        }
    }
}

#[get("/")]
async fn hello(req: HttpRequest) -> Result<HttpResponse> {
    let s = Index::from_request(req)
        .render()
        .unwrap_or_else(|_| "Oops, template rendering error".to_string());
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[post("/")]
async fn echo(req: HttpRequest) -> Result<impl Responder> {
    Ok(web::Json(Index::from_request(req)))
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    let args = Args::parse();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let bind_result = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
    })
    .bind(&args.addr);
    let server = match bind_result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: Cannot bind the address {} ({}).\nAddress format: <ip>:<port> (e.g. --addr 0.0.0.0:8080)", args.addr, e);
            std::process::exit(1);
        }
    };
    server.run()
    .await
}
