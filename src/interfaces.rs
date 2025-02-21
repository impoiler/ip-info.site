use maxminddb::Reader;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

// Struct to hold the geolocation data
#[derive(Serialize)]
pub struct GeoData {
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub postal_code: Option<String>,
    pub time_zone: Option<String>,
    pub subdivision: Option<String>,
    pub asn: Option<u32>,
    pub organization: Option<String>,
}

#[derive(Serialize)]
pub struct Error {
    pub error: String,
    pub code: Option<f64>,
}

#[derive(Deserialize)]
pub struct BatchLookupRequest {
    pub ips: Vec<String>,
}

#[derive(Serialize)]
pub struct BatchLookupResponse {
    pub results: HashMap<String, GeoData>,
    pub errors: HashMap<String, String>,
}

// Load the MaxMind databases
#[derive(Clone)]
pub struct Databases {
    pub city: Arc<Reader<Vec<u8>>>,
    pub asn: Option<Arc<Reader<Vec<u8>>>>,
}
