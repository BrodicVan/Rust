use crate::db_access::user::*;
use crate::error::MyError;
use crate::models::user::{CreateUser, UpdateUser, LoginUser, RegUser};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_users(app_state: web::Data<AppState>)-> Result<HttpResponse, MyError>{
    get_all_users_db(&app_state.db)
        .await
        .map(|users| HttpResponse::Ok().json(users))
}

pub async fn get_user_byId(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    let id = params.into_inner();
    get_user_byId_db(&app_state.db, id)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn post_new_user(
    new_user: web::Json<CreateUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_user_db(&app_state.db, CreateUser::from(new_user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn update_user_details(
    app_state: web::Data<AppState>,
    update_user: web::Json<UpdateUser>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    update_user_details_db(&app_state.db, id, UpdateUser::from(update_user),)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
)->Result<HttpResponse,MyError>{
    let id = params.into_inner();
    delete_user_db(&app_state.db, id)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn post_login_request(
    login_user: web::Json<LoginUser>,
    app_state: web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    post_login_request_db(&app_state.db, LoginUser::from(login_user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn post_reg_request(
    reg_user: web::Json<RegUser>,
    app_state: web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    post_reg_request_db(&app_state.db, RegUser::from(reg_user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
}