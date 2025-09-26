use actix_web::{HttpResponse, Responder};

#[actix_web::get("/{path:.*}")]
async fn read_page() -> impl Responder {
    HttpResponse::Ok().body("
        <html>
            <head>
                <title>Path</title>
                <meta charset='utf-8'>
            </head>
            <body>
                Test
            </body>
        </html>
    ")
}

#[actix_web::main]
async fn main() {
    use actix_web::{App, HttpServer};
    HttpServer::new(|| {
        App::new()
            .service(read_page)
    })
        .bind("127.0.0.1:8080").unwrap()
        .run().await.unwrap()
}
