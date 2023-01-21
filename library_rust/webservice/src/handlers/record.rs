use actix_web::{web, HttpResponse};

use crate::{state::AppState, error::MyError, db_access::record::{get_all_records_by_user_id_db, return_book_db, get_all_records_db}, models::book::ReturnBook};

pub async fn get_all_records_by_user_id(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    let user_id = params.into_inner();
    get_all_records_by_user_id_db(&app_state.db,user_id)
        .await
        .map(|records| HttpResponse::Ok().json(records))
}

pub async fn get_all_records(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("进来这里了");
    get_all_records_db(&app_state.db)
        .await
        .map(|records| HttpResponse::Ok().json(records))
}

pub async fn return_book(
    app_state: web::Data<AppState>,
    return_book: web::Json<ReturnBook>,
) -> Result<HttpResponse, MyError> {
    
    return_book_db(&app_state.db,ReturnBook::from(return_book))
        .await
        .map(|records| HttpResponse::Ok().json(records))
}