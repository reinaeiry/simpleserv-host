use axum::{
  extract::Query,
  http::StatusCode,
  response::{Html, IntoResponse},
  routing::{get, post},
  Router,
};
use std::{collections::HashMap, net::SocketAddr};
use tokio::fs;

const CLIENT_IP: &str = "http://192.168.1.105:3030";

#[tokio::main]
async fn main() {
  let app = Router::new()
      .route("/", get(dashboard))
      .route("/status", get(fetch_status))
      .route("/exec", get(forward_command))
      .route("/power/shutdown", post(forward_shutdown))
      .route("/power/restart", post(forward_restart));

  let addr = SocketAddr::from(([0, 0, 0, 0], 6969));
  println!("Control panel running at http://{}", addr);

  axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
      .await
      .unwrap();
}

async fn dashboard() -> impl IntoResponse {
    match fs::read_to_string("dashboard.html").await {
        Ok(contents) => Html(contents).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Dashboard not found",
        )
            .into_response(),
    }
}


async fn fetch_status() -> impl IntoResponse {
  match reqwest::get(&format!("{}/status", CLIENT_IP)).await {
      Ok(resp) => match resp.text().await {
          Ok(body) => Html(body).into_response(),
          Err(_) => (StatusCode::BAD_GATEWAY, "Failed to parse response").into_response(),
      },
      Err(_) => (StatusCode::BAD_GATEWAY, "Failed to reach client").into_response(),
  }
}

async fn forward_command(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
  if let Some(cmd) = params.get("cmd") {
      let url = format!("{}/exec", CLIENT_IP);
      match reqwest::Client::new()
          .post(&url)
          .json(&serde_json::json!({ "cmd": cmd }))
          .send()
          .await
      {
          Ok(res) => {
              match res.json::<HashMap<String, String>>().await {
                  Ok(map) => Html(map.get("output").cloned().unwrap_or_default()).into_response(),
                  Err(_) => (StatusCode::BAD_GATEWAY, "Invalid response").into_response(),
              }
          }
          Err(_) => (StatusCode::BAD_GATEWAY, "Failed to reach client").into_response(),
      }
  } else {
      (StatusCode::BAD_REQUEST, "Missing command").into_response()
  }
}

async fn forward_shutdown() -> impl IntoResponse {
  let url = format!("{}/power/shutdown", CLIENT_IP);
  let _ = reqwest::Client::new().post(&url).send().await;
  "Shutdown signal sent to client"
}

async fn forward_restart() -> impl IntoResponse {
  let url = format!("{}/power/restart", CLIENT_IP);
  let _ = reqwest::Client::new().post(&url).send().await;
  "Restart signal sent to client"
}
