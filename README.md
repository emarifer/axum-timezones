# Axum Time Zones

### Simple Rest API for handling dates and times with different time zones, developed with Axum/Rust.

To run the application (in development mode):

```bash
$ cargo run # or cargo watch -q -c -w src/ -x run (cargo-watch must be installed on the system)
```

To build the project for production and minimize its size:

```bash
$ cargo build --release
```

Runs the app in the development mode.<br>
Open [http://localhost:8080/healthchecker](http://localhost:8080/healthchecker) to view it in the browser.

The page will reload if you make edits.
