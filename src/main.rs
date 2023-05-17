use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::RwLock;

type Dates = Arc<RwLock<Vec<DateTime<Utc>>>>;

// Solicitar los datos formateado den stdout con:
// curl http://localhost:8080/healthchecker | json_pp
// VER: https://mkyong.com/web/how-to-pretty-print-json-output-in-curl/
async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str =
        "Building a simple API in Rust using Axum to handle time zones via Chrono-tz";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

fn start_datetime_db() -> Dates {
    Arc::new(RwLock::new(Vec::new()))
}

#[derive(Deserialize)]
struct DateTimeRequest {
    date_time: String,
}

async fn create_datetime_handler(
    State(db): State<Dates>,
    Json(body): Json<DateTimeRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let dt = match DateTime::parse_from_rfc3339(&body.date_time) {
        Ok(value) => value,
        Err(e) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Could not parse datetime: {}", e),
            });

            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    db.write().await.push(dt.with_timezone(&Utc));

    let success_response = json!({
        "status": "success".to_string(),
        "message": format!("Added date with timezone: {} as UTC", dt.timezone()),
    });

    Ok((StatusCode::CREATED, Json(success_response)))
}

async fn fetch_datetime_handler(
    State(db): State<Dates>,
    Path(timezone): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Minimal url-encoding fix for parsing time zones like Africa/Algiers etc.
    let time_zone = timezone.replace("%2F", "/");

    let tz: Tz = match time_zone.parse() {
        Ok(value) => value,
        Err(e) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Could not parse timezone: {}", e),
            });

            return Err((StatusCode::BAD_REQUEST, Json(error_response)));
        }
    };

    let data = db
        .read()
        .await
        .iter()
        .map(|dt| dt.with_timezone(&tz).to_rfc3339())
        .collect::<Vec<_>>();

    match serde_json::to_string(&data) {
        Ok(_) => {
            let success_response = json!({
                "status": "success".to_string(),
                "data": data,
            });

            Ok((StatusCode::OK, Json(success_response)))
        }
        Err(e) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Could not serialize json: {}", e),
            });

            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = start_datetime_db();

    let app = Router::new()
        .route("/healthchecker", get(health_checker_handler))
        .route("/create", post(create_datetime_handler))
        .route("/fetch/:query_tz", get(fetch_datetime_handler))
        .with_state(db);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");

    println!("🚀 Server started successfully!!\n");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

/*
 * CREAR DATETIME CON CURL:
 * curl -X POST http://localhost:8080/create -d '{"date_time": "2023-05-17T17:50:34+02:00"}' -H "content-type: application/json" | json_pp
 *
 * RECUPERAR LA LISTA DE DATETIME CREADOS CON CURL (en la timezone UTC):
 * curl http://localhost:8080/fetch/UTC | json_pp
 *
 * RECUPERAR LA LISTA DE DATETIME CREADOS CON CURL (en un timezone determinada):
 * curl http://localhost:8080/fetch/Europe%2FMadrid | json_pp
 *
 * OBTENER LA DATETIME ACTUAL Y/0 CONVERTIRLA A LOS DIFERENTES FORMATOS:
 * https://it-tools.tech/date-converter
 *
 * LISTADO DE ZONAS HORARIAS:
 * https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html
 *
 * REFERENCIAS. VER:
 *
 * https://blog.logrocket.com/timezone-handling-in-rust-with-chrono-tz/
 * https://github.com/zupzup/rust-timezones-chrono-tz
 *
 * https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html
 * https://docs.rs/tokio/latest/tokio/sync/struct.Mutex.html
 *
 * https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/
 * https://github.com/wpcodevo/simple-api-rust-axum
 * https://codevoweb.com/?s=axum
 *
 * https://github.com/jeremychone-channel/rust-axum-course
 *
 * https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/201
 *
 * https://mkyong.com/web/how-to-pretty-print-json-output-in-curl/
 * https://linuxize.com/post/curl-rest-api/?utm_content=cmp-true
 * https://www.baeldung.com/curl-rest
 *
 * https://doc.rust-lang.org/rust-by-example/flow_control/let_else.html
 */
