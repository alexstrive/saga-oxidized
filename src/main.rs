use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder, Scope};

mod services;

struct ServerInfo {
  port: String,
  address: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let server_info = ServerInfo {
    port: std::env::var("PORT").unwrap_or("3050".to_owned()),
    address: std::env::var("ADDRESS").unwrap_or("localhost".to_owned()),
  };

  let url = format!("{}:{}", server_info.address, server_info.port);

  HttpServer::new(|| {
    let app = App::new();
    app.service(services::templates())
  })
  .bind(url)?
  .run()
  .await
}
