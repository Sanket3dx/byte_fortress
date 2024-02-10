use actix_cors::Cors;
use actix_web::web::PayloadConfig;
use actix_web::{web, App, HttpServer};
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;
use open;

mod controllers;
mod data_schema;
mod rocks_db_operations;
mod routes;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );

    let start_message = Style::new().bold().green().apply_to("[INFO]").to_string()
        + "Byte Fortress is starting ......";
    pb.set_message(start_message);

    thread::sleep(Duration::from_secs(4));
    let end_message = Style::new().bold().green().apply_to("[INFO]").to_string()
        + "Byte Fortress is started successfully ✅";
    pb.finish_with_message(end_message);
    open::that("http://localhost:8080").expect("Failed to open browser");
    let payload_config = PayloadConfig::new(usize::MAX).limit(usize::MAX);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(payload_config.clone()) // Set payload size limit
            .wrap(cors)
            .configure(routes::files_routes::configure_files_routes)
            .route("/", web::get().to(controllers::file_controller::index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    

    Ok(())
}
