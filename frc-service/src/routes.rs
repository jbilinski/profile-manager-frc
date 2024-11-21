// Web application routes mapping url paths to handlers

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/generate_resume")
            .route(web::post().to(crate::handlers::generate_resume))
    );
}