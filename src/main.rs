use actix_web::{HttpResponse, Responder};
use tera::{Tera, Context};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERA_TEMPLATES: Tera = Tera::new("public/**/*").unwrap();
}

#[actix_web::get("/{path:[^?]*}")]
async fn read_page() -> impl Responder {
    HttpResponse::Ok().body(
        ::std::fs::read_to_string("public/html/index.html").unwrap()
    )
}
#[actix_web::get("/favicon.ico")]
async fn favicon() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(format!("{:0x?}",
        ::std::fs::read("public/img/favicon.ico").unwrap()
    ))
}

#[actix_web::main]
async fn main() {
    use actix_web::{App, HttpServer};
    return HttpServer::new(|| {
        App::new()
            .service(favicon)
            .service(read_page)
    })
        .bind("0.0.0.0").unwrap()
        .run().await.unwrap();
}
