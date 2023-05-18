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

---

Add date times (using cURL) with:

```bash
curl -v -X POST http://localhost:8080/create -d '{"date_time": "1996-12-19T16:39:57+02:00"}' -H "content-type: application/json" | json_pp
```

And fetch them, converted to a given timezone with:

```bash
// UTC
curl -v http://localhost:8080/fetch/UTC | json_pp
["1996-12-19T14:39:57+00:00"]

// UTC +01:00 [-NO DST- (No Daylight Saving Time)] (see: https://cambiohorario.com/africa/west/argelia/)
curl -v http://localhost:8080/fetch/Africa%2FAlgiers | json_pp
["1996-12-19T15:39:57+01:00"]

// UTC +01:00 [Winter time] (see: https://cambiohorario.com/eu/espana/)
curl -v http://localhost:8080/fetch/Europe%2FMadrid | json_pp
["1996-12-19T15:39:57+01:00"]

// UTC +03:00 [-NO DST- (No Daylight Saving Time)]
curl -v http://localhost:8080/fetch/Europe%2FMoscow | json_pp
["1996-12-19T17:39:57+03:00"] (see: https://cambiohorario.com/eu/moscu/)
```

Valid time zone strings can be found [here](https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html).
