use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use maxminddb::Reader;
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;

// Struct to hold the geolocation data
#[derive(Serialize)]
struct GeoData {
    country: Option<String>,
    city: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    postal_code: Option<String>,
    time_zone: Option<String>,
    subdivision: Option<String>,
}

#[derive(Serialize)]
struct Error {
    error: String,
    code: Option<f64>,
}

// Load the MaxMind database
fn load_database(path: &str) -> Arc<Reader<Vec<u8>>> {
    let reader = Reader::open_readfile(path).expect("Failed to load MaxMind database");
    Arc::new(reader)
}

// Handler for IP lookup
#[get("/lookup")]
async fn lookup(
    req: HttpRequest,
    ip_query: web::Query<HashMap<String, String>>,
    db: web::Data<Arc<Reader<Vec<u8>>>>,
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

    // Get the IP from the query parameters, or use a default
    let ip = ip_query.get("ip").unwrap_or(&self_ip).to_string();

    match ip.parse::<IpAddr>() {
        Ok(parsed_ip) => {
            // Query the MaxMind database
            match db.lookup::<maxminddb::geoip2::City>(parsed_ip) {
                Ok(city_data) => {
                    // Extract relevant fields
                    let geo_data = GeoData {
                        country: city_data.country.and_then(|c| c.iso_code.map(String::from)),
                        city: city_data
                            .city
                            .and_then(|c| c.names.and_then(|n| n.get("en").map(|s| s.to_string()))),
                        latitude: city_data.location.clone().and_then(|loc| loc.latitude),
                        longitude: city_data.location.clone().and_then(|loc| loc.longitude),
                        postal_code: city_data.postal.and_then(|p| p.code.map(String::from)),
                        time_zone: city_data
                            .location
                            .and_then(|loc| loc.time_zone.map(String::from)),
                        subdivision: city_data.subdivisions.and_then(|subs| {
                            subs.first().and_then(|sub| sub.iso_code.map(String::from))
                        }),
                    };
                    HttpResponse::Ok().json(geo_data) // Return data as JSON
                }
                Err(_) => HttpResponse::BadRequest().body("Invalid IP or not found in database"),
            }
        }
        Err(_) => HttpResponse::BadRequest().json(Error {
            error: ("Invalid IP format").to_string(),
            code: None,
        }),
    }
}

#[get("/")]
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

// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Path to your MaxMind GeoLite2 database
    let db_path = "./GeoLite2-City.mmdb";
    let database = load_database(db_path);

    println!("Starting server on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone())) // Share database across threads
            .service(get_ip) // Ad
            .service(lookup) // Register the lookup endpoint
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
