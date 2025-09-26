use actix_web::{HttpResponse, Responder};

#[actix_web::get("/{path:.*}")]
async fn read_page() -> impl Responder {
    return HttpResponse::Ok().body("
        <html>
            <head>
                <title>Path</title>
                <meta charset='utf-8'>
            </head>
            <body>
                Hello everybody, I am Iscra! Call me Iscra-san or Iscra-chan. :)
            </body>
        </html>
    ");
}

#[actix_web::main]
async fn main() {
    use actix_web::{App, HttpServer};
    return HttpServer::new(|| {
        App::new()
            .service(read_page)
    })
        .bind("127.0.0.1:8080").unwrap()
        .run().await.unwrap();
}
