# erwa-example
Example implementation of https://erwabook.com/intro/ with updated dependencies

**frontend**
```toml
[dependencies]
seed = "0.8.0"
wasm-bindgen = "0.2.68"
web-sys = "0.3.45"
```

**backend**
```toml
[dependencies]
diesel = { version = "1.4.5", features = ["sqlite"] }
rocket = "0.4.5"
serde = { version = "1.0.117", features = ["derive"] }
rocket_cors = { version = "0.5.2", default-features = false }

[dependencies.rocket_contrib]
version = "0.4.5"
default-features = false
features = ["json"]
```
