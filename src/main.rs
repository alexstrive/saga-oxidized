use actix_web::{get, middleware, App, Error, HttpResponse, HttpServer, Responder, Scope};
// use clap::{App, Arg};

mod services;

struct ServerInfo {
  port: String,
  address: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let matches = clap::App::new("Saga Choreographer")
    .version("0.1.0")
    .author("Alex S. <alexstrive@pm.me>")
    .arg(
      clap::Arg::with_name("port")
        .short("p")
        .long("port")
        .default_value("8080"),
    )
    .arg(
      clap::Arg::with_name("bind-address")
        .short("b")
        .long("bind-address")
        .default_value("localhost"),
    )
    .get_matches();

  let server_info = ServerInfo {
    port: matches.value_of("port").unwrap().to_owned(),
    address: matches.value_of("bind-address").unwrap().to_owned(),
  };

  let url = format!("{}:{}", server_info.address, server_info.port);

  HttpServer::new(|| {
    App::new()
    // .configure(services::kafka)
    // .service(services::templates())
  })
  .bind(url)?
  .run()
  .await
}
