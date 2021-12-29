use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/courses").route("/", web::post().to(new_course)));
    /*
    curl -H "Content-Type: application/json" -X POST -d '{"teacher_id":1, "name":"First course"}' "127.0.0.1:3000/courses/"
    */
}
