# Atopile component server

# Description
Alternative backend to the official Atopile component search-engine.

## Tech stack
Rust, Axum, SQLX, Docker

# Launch the APP
```
cargo run -- -d postgresql://atopile:<PWD>@192.168.3.22:5430/atopile-jlcpcb
```

# Development with VSCode Devcontainer
Development of the Atopile Component Server is done with (VSCode Devcontainers)[https://code.visualstudio.com/docs/devcontainers/containers].
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

post request request for parts
```
localhost:3001/jlc
```

example request body
```
{"designator_prefix": "R", "mpn": "generic_resistor", "type": "resistor", "value": {"unit": "megaohm", "min_val": 5.01534, "max_val": 5.1166599999999995, "nominal": 5.066,
"tolerance": 0.050659999999999705, "tolerance_pct": 3.9999999999999943}}
```

```
{"designator_prefix": "C", "mpn": "generic_capacitor", "type": "capacitor", "value": {"unit": "nanofarad", "min_val": 80.0, "max_val": 120.0, "nominal": 100.0, "tolerance": 
20.0, "tolerance_pct": 20.0}, "package": "0402"}
```

```
{"designator_prefix": "C", "mpn": "generic_inductor", "type": "inductor", "value": {"unit": "nanohenry", "min_val": 80.0, "max_val": 120.0, "nominal": 9030.0, "tolerance": 
20.0,"tolerance_pct": 10}}
```
## Demo endpoint (running on raspberry pi)
[https://jlcparts.vfive.dev/docs](https://jlcparts.vfive.dev/docs)