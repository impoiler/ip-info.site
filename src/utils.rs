use crate::interfaces::{Databases, GeoData};
use maxminddb::Reader;
use std::net::IpAddr;
use std::sync::Arc;

pub fn load_databases() -> Databases {
    let city_reader = Reader::open_readfile("./data/GeoLite2-City.mmdb")
        .expect("Failed to load MaxMind City database");

    let asn_reader = Reader::open_readfile("./data/GeoLite2-ASN.mmdb")
        .ok()
        .map(Arc::new);

    Databases {
        city: Arc::new(city_reader),
        asn: asn_reader,
    }
}

pub fn lookup_ip(ip: IpAddr, dbs: &Databases) -> Result<GeoData, String> {
    let city_data = dbs
        .city
        .lookup::<maxminddb::geoip2::City>(ip)
        .map_err(|e| e.to_string())?;

    let (asn, organization) = if let Some(asn_db) = &dbs.asn {
        match asn_db.lookup::<maxminddb::geoip2::Asn>(ip) {
            Ok(asn_data) => (
                asn_data.autonomous_system_number,
                asn_data.autonomous_system_organization.map(String::from),
            ),
            Err(_) => (None, None),
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
        subdivision: city_data
            .subdivisions
            .and_then(|subs| subs.first().and_then(|sub| sub.iso_code.map(String::from))),
        asn,
        organization,
    })
}
