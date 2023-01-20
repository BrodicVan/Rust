use crate::handlers::{course::*, user::*,general::health_check_handler};

use actix_web::web;
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{teacher_id}", web::get().to(get_courses_for_teacher))
            .route(
                "/{teacher_id}/{course_id}",
                web::get().to(get_course_detail),
            )
            .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
            .route(
                "/{teacher_id}/{course_id}",
                web::put().to(update_course_details),
            ),
    );
}

pub fn user_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(post_new_user))
            .route("/", web::get().to(get_all_users))
            .route("/{user_id}", web::get().to(get_user_byId))
            .route("/{user_id}", web::put().to(update_user_details))
            .route("/{user_id}", web::delete().to(delete_user))
    );
}
