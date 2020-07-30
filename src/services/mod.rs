use actix_web::{web, Scope};

async fn index() -> String {
  format!("Hello, world! {}", 123)
}
async fn post() -> String { format!("Post accessed") }

pub fn templates() -> Scope {
  web::scope("/templates")
    .route("/add", web::get().to(index))
    .route("/edit", web::post().to(post))
}

// pub fn kafka(cfg: &mut web::ServiceConfig) {
//   cfg.service(web::scope("/kafka").route("/subscribe", web::get().to(|| HttpResponse::Ok())));
// }
