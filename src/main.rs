pub mod iscro_http_handler {
    /*
    # MODULES:
    */
    use std::collections::HashMap;

    use actix_web::{
        Responder, HttpResponse,
        web::{Path, Query, Data, Json, Redirect, Either},
    };
    use serde::{Serialize, Deserialize};
    use sqlx::{PgPool, FromRow};
    use tera::{Tera, Context};

    use lazy_static;
    lazy_static::lazy_static! {
        static ref TERA_TEMPLATE: Tera = Tera::new("public/html/**/*").unwrap();
    }

    /*
    # STRUCTS FOR POSTGRESQL:
    */
    #[derive(Debug, FromRow, Serialize, Deserialize)]
    struct Page {
        title: String,
        content: String,
        pw_for_edit: String,
        pw_for_logs: String,
        pw_for_deletion: String,
    }

    /*
    # HTTP GET HANDLER:
    */
    #[actix_web::get("{path:.*}")]
    pub async fn get(
        path: Path<String>,
        uri_query: Query<HashMap<String, String>>,
        db_pool: Data<PgPool>,
    ) -> Either<impl Responder, impl Responder> {
        let path = path.into_inner();
        let uri_query = uri_query.into_inner();
        let db_query_result = sqlx::query!(
            "SELECT title, content FROM pages WHERE url = $1",
            path
        )   .fetch_one(db_pool.get_ref())
            .await;

        if uri_query.is_empty() {
            let mut context = Context::new();
            return Either::Right(
                match db_query_result {
                    Ok(db_query) => {
                        context.insert("title", &db_query.title);
                        context.insert("content", &db_query.content);
                        HttpResponse::Ok
                    },
                    Err(_) => HttpResponse::NotFound,
                } ().body(
                    TERA_TEMPLATE.render("index.html", &context).unwrap()
                )
            );
        }
        if uri_query.contains_key("create-page") {
            return match db_query_result {
                Ok(_) => Either::Left(Redirect::to("?edit-page")),
                Err(_) => Either::Right(
                    HttpResponse::Ok().body(
                        ::std::fs::read_to_string("public/html/create-page.html").unwrap()
                    )
                ),
            };
        }
        if uri_query.contains_key("edit-page") {
            return match db_query_result {
                Ok(db_query) => Either::Right(
                    HttpResponse::Ok().body(
                        TERA_TEMPLATE.render("edit-page.html", &{
                            let mut context = Context::new();
                            context.insert("title", &db_query.title);
                            context.insert("content", &db_query.content);
                            context
                        }).unwrap()
                    )
                ),
                Err(_) => Either::Left(Redirect::to("?create-page")),
            };
        }
        if uri_query.contains_key("read-logs") {
            return Either::Right(HttpResponse::Ok().body(
                ::std::fs::read("public/html/read-logs.html").unwrap()
            ));
        }
        if uri_query.contains_key("delete-page") {
            return Either::Right(HttpResponse::Ok().body(
                ::std::fs::read("public/html/delete-page.html").unwrap()
            ));
        }
        Either::Left(Redirect::to(path))
    }

    /*
    # HTTP POST HANDLER:
    */

    #[actix_web::post("/{path:.*}")]
    pub async fn post(
        path: Path<String>,
        uri_query: Query<HashMap<String, String>>,
        json: Json<Page>,
        db_pool: Data<PgPool>,
    ) -> impl Responder {
        let path = path.into_inner();
        let uri_query = uri_query.into_inner();
        let json = json.into_inner();
        let db_query_result = sqlx::query!(
            "SELECT pw_for_edit, pw_for_logs, pw_for_deletion FROM pages WHERE url = $1",
            path
        )   .fetch_one(db_pool.get_ref())
            .await;

        if uri_query.contains_key("page-created") {
            return Json(
                sqlx::query!(
                    "INSERT INTO pages (
                        url, title, content, pw_for_edit, pw_for_logs, pw_for_deletion
                    ) VALUES ($1, $2, $3, $4, $5, $6)",
                    path, json.title, json.content, json.pw_for_edit, json.pw_for_logs, json.pw_for_deletion
                )
                    .execute(db_pool.get_ref())
                    .await.is_ok()
            );
        }
        if uri_query.contains_key("page-edited") {
            if json.pw_for_edit != db_query_result.unwrap().pw_for_edit {
                return Json(false);
            }
            return Json(
                sqlx::query!(
                    "UPDATE pages SET title=$2, content=$3 WHERE url=$1;",
                    path, json.title, json.content
                )
                    .execute(db_pool.get_ref())
                    .await.is_ok()
            );
        }
        return Json(false);
    }
}
#[actix_web::main]
async fn main() {
    use actix_web::{
        App, HttpServer,
        web::Data,
    };
    use actix_files::Files;
    use sqlx::{postgres::PgPoolOptions};

    unsafe { ::std::env::set_var("RUST_LOG", "debug") };
    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:postgres@localhost:5432/ip-grabber")
        .await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(Files::new("/public", "public"))
            .service(iscro_http_handler::get)
            .service(iscro_http_handler::post)
    })
        .bind("0.0.0.0:8080").unwrap()
        .run().await.unwrap();
}
