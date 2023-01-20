use actix_web::{web, HttpResponse};

use crate::{state::AppState, error::MyError, db_access::book::get_all_books_db};

pub async fn get_all_books(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    
    get_all_books_db(&app_state.db)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}