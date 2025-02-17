use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::NamedFile;
use maxminddb::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::path::PathBuf;

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
    asn: Option<u32>,
    organization: Option<String>,
}

#[derive(Serialize)]
struct Error {
    error: String,
    code: Option<f64>,
}

#[derive(Deserialize)]
struct BatchLookupRequest {
    ips: Vec<String>,
}

#[derive(Serialize)]
struct BatchLookupResponse {
    results: HashMap<String, GeoData>,
    errors: HashMap<String, String>,
}

// Load the MaxMind databases
#[derive(Clone)]
struct Databases {
    city: Arc<Reader<Vec<u8>>>,
    asn: Option<Arc<Reader<Vec<u8>>>>,
}

fn load_databases() -> Databases {
    let city_reader = Reader::open_readfile("./data/GeoLite2-City.mmdb")
        .expect("Failed to load MaxMind City database");
    
    let asn_reader = Reader::open_readfile("./data/GeoLite2-ASN.mmdb").ok().map(Arc::new);
    
    Databases {
        city: Arc::new(city_reader),
        asn: asn_reader,
    }
}

fn lookup_ip(ip: IpAddr, dbs: &Databases) -> Result<GeoData, String> {
    let city_data = dbs.city.lookup::<maxminddb::geoip2::City>(ip)
        .map_err(|e| e.to_string())?;
    
    let (asn, organization) = if let Some(asn_db) = &dbs.asn {
        match asn_db.lookup::<maxminddb::geoip2::Asn>(ip) {
            Ok(asn_data) => (
                asn_data.autonomous_system_number,
                asn_data.autonomous_system_organization.map(String::from)
            ),
            Err(_) => (None, None)
        }
    } else {
        (None, None)
    };

    Ok(GeoData {
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
        asn,
        organization,
    })
}

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
        Ok(parsed_ip) => {
            match lookup_ip(parsed_ip, &dbs) {
                Ok(geo_data) => HttpResponse::Ok().json(geo_data),
                Err(e) => HttpResponse::BadRequest().body(e),
            }
        }
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
            Ok(parsed_ip) => {
                match lookup_ip(parsed_ip, &dbs) {
                    Ok(geo_data) => {
                        response.results.insert(ip.clone(), geo_data);
                    }
                    Err(e) => {
                        response.errors.insert(ip.clone(), e);
                    }
                }
            }
            Err(_) => {
                response.errors.insert(ip.clone(), "Invalid IP format".to_string());
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

    println!("Starting server on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(databases.clone())) // Share database across threads
            .service(index) // Serve HTML documentation at root
            .service(get_ip) // IP endpoint
            .service(lookup) // Register the lookup endpoint
            .service(batch_lookup)
            .service(web::scope("").wrap(
                actix_web::middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET, POST"))
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
