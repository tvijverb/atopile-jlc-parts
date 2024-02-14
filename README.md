# Atopile component server

# Description
Alternative backend to the official Atopile component search-engine.

## Tech stack
Rust, Axum, Pola.rs, Docker

## Server start
Clone this repository to your local computer and run:
```bash
docker run -p 3001:3000 -d $(docker build -q -f dockerfile.prd .)
```

or if you prefer docker compose
```bash
docker compose up
```

## Endpoints
0.0.0.0/docs => swagger page
0.0.0.0/jlc => post request request for parts

example request body
```
{"designator_prefix": "R", "mpn": "generic_resistor", "type": "resistor", "value": {"unit": "megaohm", "min_val": 5.01534, "max_val": 5.1166599999999995, "nominal": 5.066,
"tolerance": 0.050659999999999705, "tolerance_pct": 3.9999999999999943}}
```

## Demo endpoint (running on raspberry pi)
[https://jlcparts.vfive.dev/docs](https://jlcparts.vfive.dev/docs)