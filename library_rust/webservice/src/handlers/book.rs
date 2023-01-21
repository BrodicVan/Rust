use actix_web::{web, HttpResponse};

use crate::{state::AppState, error::MyError, db_access::book::{get_all_books_db, borrow_book_db, add_book_db, edit_book_db, delete_book_db}, models::book::{BorrowBook, CreateBook, UpdateBook, DeleteBook}};

pub async fn get_all_books(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    
    get_all_books_db(&app_state.db)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn borrow_book(
    app_state: web::Data<AppState>,
    borrow_book: web::Json<BorrowBook>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let user_id = params.into_inner();
    borrow_book_db(&app_state.db, user_id, BorrowBook::from(borrow_book),)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn add_book(
    new_book: web::Json<CreateBook>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    add_book_db(&app_state.db, CreateBook::from(new_book))
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn edit_book(
    app_state: web::Data<AppState>,
    update_book: web::Json<UpdateBook>,
) -> Result<HttpResponse, MyError> {
    edit_book_db(&app_state.db,  update_book.into())
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn delete_book(
    app_state: web::Data<AppState>,
    delete_book: web::Json<DeleteBook>,
)->Result<HttpResponse,MyError>{
    delete_book_db(&app_state.db, delete_book.into())
        .await
        .map(|book| HttpResponse::Ok().json(book))
}
