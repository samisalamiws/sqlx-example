extern crate dotenv;
use dotenv::dotenv;
use std::env;

use std::sync::Mutex;
use sqlx;
use actix_web::web::Data;
use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};


/// CREATE TABLE public.task (
///	description varchar NOT NULL,
///	priority int NOT NULL,
///	id int NOT NULL GENERATED ALWAYS AS IDENTITY
/// );
/// 
#[derive(Debug)]
struct Task {
    description: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn list_view(data: Data<Mutex<MyPool>>) -> impl Responder {
    let my_pool = &data.lock().unwrap();

    let tasks = sqlx::query_as!(Task,
        "
        SELECT description from public.task
        "
    )
    .fetch_all(&my_pool.pool) // -> Vec<Task>
    .await.unwrap();
   
    HttpResponse::Ok().body(format!("Hey there! {:?}", tasks))
}

struct MyPool {
     pool: sqlx::Pool<sqlx::Postgres>,
 }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await.unwrap();

    let pool_struct = MyPool{ pool };

    let data = Data::new(Mutex::new(pool_struct));

    HttpServer::new(move|| {
        App::new()
            .app_data(Data::clone(&data))
            .route("/", web::get().to(list_view))
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await 
}