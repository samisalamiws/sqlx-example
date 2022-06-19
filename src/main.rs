use std::sync::Mutex;
use sqlx;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello(data: Data<Mutex<MyPool>>) -> impl Responder {
    let my_pool = &data.lock().unwrap();
    let row: (String,) = sqlx::query_as("SELECT name from public.test")
        .fetch_one(&my_pool.pool).await.unwrap(); 
    HttpResponse::Ok().body(format!("Hey there! {}", row.0.as_str()))
}

struct MyPool {
     pool: sqlx::Pool<sqlx::Postgres>,     
 }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/postgres").await.unwrap();

    let pool_struct = MyPool{ pool };

    let data = Data::new(Mutex::new(pool_struct));

    HttpServer::new(move|| {
        App::new()
            .app_data(Data::clone(&data))
            .route("/hello", web::get().to(manual_hello))
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await 
}