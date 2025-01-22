# IP Lookup Service

> ⚠️ **Note**: This project is currently under active development. Features and APIs may change.

A high-performance IP geolocation service built with Rust that provides detailed information about IP addresses using the MaxMind GeoLite2 database.

## Features

- 🚀 Fast and efficient IP address lookups
- 🌍 Geolocation data including city, country, and coordinates
- 🔒 Self-hosted solution
- 🛠 Built with Rust for maximum performance
- 🐳 Docker support for easy deployment

## Installation

1. Clone the repository:
```bash
git clone https://github.com/impoiler/ip-info.site.git
cd ip-info.site
```

2. Build the project:
```bash
cargo build --release
```

3. Make sure you have the GeoLite2 City database (`GeoLite2-City.mmdb`) in the project root directory.

## Usage

### Running Locally

```bash
cargo run --release
```

The service will start on `http://localhost:8080` by default.

### Using Docker

1. Build the Docker image:
```bash
docker build -t ip-lookup-service .
```

2. Run the container:
```bash
docker run -p 8080:8080 ip-lookup-service
```

## API Endpoints

### GET /ip/:ip_address

Returns geolocation information for the specified IP address.

Example request:
```bash
curl http://localhost:8080/ip/8.8.8.8
```

Example response:
```json
{
    "ip": "8.8.8.8",
    "country": "United States",
    "city": "Mountain View",
    "latitude": 37.4223,
    "longitude": -122.0847
}
```

## Dependencies

- actix-web (4.0) - Web framework
- maxminddb (0.17) - MaxMind DB reader
- serde (1.0) - Serialization framework
- serde_json (1.0.133) - JSON support

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [MaxMind](https://www.maxmind.com) for providing the GeoLite2 database
- The Rust community for excellent documentation and tools

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Support

For support, please open an issue in the GitHub repository.
