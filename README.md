# IP Geolocation API

A fast and reliable service that provides IP address geolocation information using the MaxMind GeoLite2 database. Get detailed location data for any IP address or discover your own IP address with our simple API endpoints.

> ⚠️ **Note**: This project is currently under active development. Features and APIs may change.

## Features

- Get your current IP address
- Lookup geolocation data for any IP address
- Fast response times
- Built with Rust for optimal performance
- Uses MaxMind GeoLite2 database for accurate geolocation data

## API Endpoints

### Get Your IP Address
```
GET /ip
```
Returns your current IP address in plain text format.

### IP Geolocation Lookup
```
GET /lookup?ip=x.x.x.x
```
Returns geolocation data for the specified IP address. If no IP is provided, it uses your current IP.

#### Response Example
```json
{
    "country": "US",
    "city": "San Francisco",
    "latitude": 37.7749,
    "longitude": -122.4194,
    "postal_code": "94105",
    "time_zone": "America/Los_Angeles",
    "subdivision": "CA",
    "asn": 13335,
    "organization": "Cloudflare, Inc."
}
```

### Batch IP Lookup
```
POST /batch-lookup
```
Lookup multiple IP addresses in a single request. Supports up to 100 IPs per request.

#### Request Body
```json
{
    "ips": ["8.8.8.8", "1.1.1.1"]
}
```

#### Response Example
```json
{
    "results": {
        "8.8.8.8": {
            "country": "US",
            "city": "Mountain View",
            "latitude": 37.4056,
            "longitude": -122.0775,
            "postal_code": "94043",
            "time_zone": "America/Los_Angeles",
            "subdivision": "CA",
            "asn": 15169,
            "organization": "Google LLC"
        },
        "1.1.1.1": {
            "country": "US",
            "city": "Los Angeles",
            "latitude": 34.0522,
            "longitude": -118.2437,
            "postal_code": "90001",
            "time_zone": "America/Los_Angeles",
            "subdivision": "CA",
            "asn": 13335,
            "organization": "Cloudflare, Inc."
        }
    },
    "errors": {}
}
```

## Running with Docker

### Option 1: Using Docker Compose (Recommended)
```bash
docker compose up -d
```
This will build the image and start the container in detached mode. To stop the service:
```bash
docker compose down
```

### Option 2: Using Docker directly

#### Build the Image
```bash
docker build -t ip-info-site .
```

#### Run the Container
```bash
docker run -p 8085:8085 -d ip-info-site
```

The API will be available at `http://localhost:8085`

## Development

This project is built with Rust and uses:
- Rust 1.82 or later
- MaxMind GeoLite2 database
- Distroless container for minimal runtime footprint

## Author

Created by Suresh

## Repository

View the source code on [GitHub](https://github.com/impoiler/ip-info.site)
