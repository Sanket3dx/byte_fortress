use actix_web::web;

use crate::controllers::file_controller::*;

pub fn configure_files_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/file")
            .route(web::get().to(handle_file_get))
            .route(web::post().to(handle_file_post))
            .route(web::put().to(handle_file_put))
            .route(web::delete().to(handle_file_delete)),
    );
}
