mod interfaces;
mod utils;

use actix_files::NamedFile;
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use interfaces::{BatchLookupRequest, BatchLookupResponse, Databases, Error};
use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;
use utils::{load_databases, lookup_ip};

#[get("/lookup")]
async fn lookup(
    req: HttpRequest,
    ip_query: web::Query<HashMap<String, String>>,
    dbs: web::Data<Databases>,
) -> impl Responder {
    let self_ip = req
        .headers()
        .get("X-Real-IP")
        .and_then(|header| header.to_str().ok())
        .or_else(|| {
            req.headers()
                .get("X-Forwarded-For")
                .and_then(|header| header.to_str().ok())
                .and_then(|ip_list| ip_list.split(',').next())
        })
        .unwrap_or("Unknown")
        .to_string();

    let ip = ip_query.get("ip").unwrap_or(&self_ip).to_string();

    match ip.parse::<IpAddr>() {
        Ok(parsed_ip) => match lookup_ip(parsed_ip, &dbs) {
            Ok(geo_data) => HttpResponse::Ok().json(geo_data),
            Err(e) => HttpResponse::BadRequest().body(e),
        },
        Err(_) => HttpResponse::BadRequest().json(Error {
            error: ("Invalid IP format").to_string(),
            code: None,
        }),
    }
}

#[post("/batch-lookup")]
async fn batch_lookup(
    req: web::Json<BatchLookupRequest>,
    dbs: web::Data<Databases>,
) -> impl Responder {
    let mut response = BatchLookupResponse {
        results: HashMap::new(),
        errors: HashMap::new(),
    };

    for ip in req.ips.iter() {
        match ip.parse::<IpAddr>() {
            Ok(parsed_ip) => match lookup_ip(parsed_ip, &dbs) {
                Ok(geo_data) => {
                    response.results.insert(ip.clone(), geo_data);
                }
                Err(e) => {
                    response.errors.insert(ip.clone(), e);
                }
            },
            Err(_) => {
                response
                    .errors
                    .insert(ip.clone(), "Invalid IP format".to_string());
            }
        }
    }

    HttpResponse::Ok().json(response)
}

#[get("/ip")]
async fn get_ip(req: HttpRequest) -> impl Responder {
    let ip = req
        .headers()
        .get("X-Real-IP")
        .and_then(|header| header.to_str().ok())
        .or_else(|| {
            req.headers()
                .get("X-Forwarded-For")
                .and_then(|header| header.to_str().ok())
                .and_then(|ip_list| ip_list.split(',').next())
        })
        .unwrap_or("Unknown");

    HttpResponse::Ok().body(ip.to_string())
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "./static/index.html".into();
    Ok(NamedFile::open(path)?)
}

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let databases = load_databases();

    println!("Starting server on http://0.0.0.0:8085");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(databases.clone())) // Share database across threads
            .service(index) // Serve HTML documentation at root
            .service(get_ip) // IP endpoint
            .service(lookup) // Register the lookup endpoint
            .service(batch_lookup)
            .service(
                web::scope("").wrap(
                    actix_web::middleware::DefaultHeaders::new()
                        .add(("Access-Control-Allow-Origin", "*"))
                        .add(("Access-Control-Allow-Methods", "GET, POST")),
                ),
            )
    })
    .bind(("0.0.0.0", 8085))?
    .run()
    .await
}
