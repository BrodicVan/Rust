use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use error::MyError;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;


#[path = "../dbaccess/mod.rs"]
mod db_access;
#[path = "../error.rs"]
mod error;
#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../models/mod.rs"]
mod models;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err,_req|{
                MyError::InvalidInput("Please Provide valid Json input".to_string()).into()
            }))
            .wrap(Cors::default()
            .allow_any_origin()
            .allowed_origin_fn(|origin, _req_head| {
                return true
            })
            .allow_any_method()
            .allow_any_header()
            
            .max_age(36000))
            .configure(general_routes)
            .configure(course_routes)
            .configure(user_routes)
            .configure(login_routes)
    };

    HttpServer::new(app)
        .bind("127.0.0.1:3333")?
        .run()
        .await
}
