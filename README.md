# Atopile component server

# Description
Alternative backend to the official Atopile component search-engine.

## Tech stack
Rust, Axum, Pola.rs, Docker

# Launch the APP
```
cargo run -- -d postgresql://atopile:<PWD>@192.168.3.22:5430/atopile-jlcpcb
```


## Install
   Compiling h2 v0.4.2
Building the docker image will take +/- 10 minutes depending on your hardware.
Clone this repository to your local computer and run:

```bash
docker run -p 3001:3000 -d $(docker build -q -f dockerfile.prd .)
```

Or if you prefer docker compose
```bash
docker compose up
```

You'll need the JLC parts parquet dataframe in order to serve requests, you can get it from this repository:
```bash
https://github.com/tvijverb/jlc-duckdb-to-parquet
```


## Endpoints
swagger page
```
0.0.0.0/docs
```

post request request for parts
```
0.0.0.0/jlc
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