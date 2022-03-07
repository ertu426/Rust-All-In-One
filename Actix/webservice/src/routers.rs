use crate::handlers::{course, general};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(general::health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/courses")
            .route("/", web::post().to(course::post_new_course))
            .route("/{teacher_id}", web::get().to(course::get_courses_for_teacher))
            .route("/{teacher_id}/{course_id}", web::get().to(course::get_course_detail))
            .route("/{teacher_id}/{course_id}", web::delete().to(course::delete_course))
            .route("/{teacher_id}/{course_id}", web::put().to(course::update_course_details))
        );
}

