use crate::handlers::{user::*,book::*,record::*,general::health_check_handler};

use actix_web::web;
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}



pub fn user_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(post_new_user))
            .route("/all", web::get().to(get_all_users))
            .route("/{user_id}", web::get().to(get_user_by_id))
            .route("/{user_id}", web::put().to(update_user_details))
            .route("/{user_id}", web::delete().to(delete_user))
            .route("/{user_id}/borrow/",web::get().to(get_all_books))
            .route("/{user_id}/borrow/", web::post().to(borrow_book))
            .route("/{user_id}/return/", web::get().to(get_all_records_by_user_id))
            .route("/{user_id}/return/",web::post().to(return_book))
    );
}

pub fn login_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/login")
            .route("/", web::post().to(post_login_request))
    );
}

pub fn reg_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/reg")
            .route("/", web::post().to(post_reg_request))
    );
}

pub fn manager_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/managers")
            .route("/{manager_id}/edit_book", web::get().to(get_all_books))
            .route("/{manager_id}/edit_record", web::get().to(get_all_records))
            .route("/{manager_id}/edit_book",web::post().to(add_book))
            .route("/{manager_id}/edit_book",web::put().to(edit_book))
            .route("/{manager_id}/edit_book",web::delete().to(delete_book))
    );
}