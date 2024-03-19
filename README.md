# Atopile component server

# Description
Alternative backend to the official Atopile component search-engine.

## Tech stack
Rust, Axum, SQLX, Docker

# Development with VSCode Devcontainer
Development of the Atopile Component Server is done with VSCode Devcontainers [https://code.visualstudio.com/docs/devcontainers/containers].
1. Install the 'Dev Containers' extension in VSCode
2. Copy .env.example to .env
3. Press F1, 'Open In Container' 
4. Connect https://github.com/tvijverb/jlcpcb_scraper to the postgres docker container on port 5431 and scrape JLCPCB
5. Start Axum
```
cargo watch -x 'run'
```

## Endpoints
swagger page
```
localhost:3001/docs
```

## V1 endpoint (current)
post request request for parts
```
localhost:3001/jlc/v1
```

example request body
```
{"designator_prefix": "R", "mpn": "generic_resistor", "type": "resistor", "value": {"unit": "megaohm", "min_val": 5.01534, "max_val": 5.1166599999999995, "nominal": 5.0663}}
```

```
{"designator_prefix": "C", "mpn": "generic_capacitor", "type": "capacitor", "value": {"unit": "nanofarad", "min_val": 80.0, "max_val": 120.0, "nominal": 100.0}, "package": "0402"}
```

```
{"designator_prefix": "C", "mpn": "generic_inductor", "type": "inductor", "value": {"unit": "nanohenry", "min_val": 80.0, "max_val": 120.0, "nominal": 100.0}}
```

## V2 endpoint (in development)
TODO

## Demo endpoint (running on raspberry pi)
[https://jlcparts.vfive.dev/docs](https://jlcparts.vfive.dev/docs)