use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web::spa;

use shelly_web::{archive_service, routes};
use std::thread;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let memory = false;
    let options = shelly_web::options::get_run_options();
    let bind: (String, u16) = (String::from("0.0.0.0"), options.port);
    let cancel = false;
    if let Some(storage_size) = options.archive {
        let cloned_options = options.clone();
        match archive_service::init_archive(memory) {
            Ok(connection) => {
                thread::spawn(move || {
                    archive_service::archive_service(
                        connection,
                        storage_size,
                        &cloned_options,
                        &cancel,
                    );
                });
            }
            Err(err) => return Err(err),
        }
    }

    println!("starting server at http://{}:{}", &bind.0, &bind.1);
    HttpServer::new(move || {
        let mut api_scope = web::scope("/api")
            .route("/shelly", web::to(routes::proxy_api_call))
            .route("/version", web::to(routes::get_version));

        let mut app = App::new()
            .wrap(Logger::default().log_target("@"))
            .app_data(web::Data::new(options.clone()));

        if options.archive.is_some() {
            app = app.app_data(web::Data::new(routes::MemoryState { memory }));
            api_scope = api_scope.route("/archive", web::to(routes::archive_get_entries));
        }

        app.service(api_scope).service(
            spa()
                .index_file("./wwwroot/index.html")
                .static_resources_location("./wwwroot")
                .finish(),
        )
    })
    .workers(1)
    .bind(bind)?
    .run()
    .await?;

    Ok(())
}
