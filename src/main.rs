use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web::spa;

use shelly_web::{archive, routes};
use std::thread;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let memory = false;
    let options = shelly_web::options::get_run_options();
    let bind: (String, u16) = (String::from("127.0.0.1"), options.port);
    let cancel = false;
    if let Some(storage_size) = options.archive {
        let cloned_options = options.clone();
        match archive::init_archive(memory) {
            Ok(connection) => {
                thread::spawn(move || {
                    archive::archive_service(
                        connection,
                        storage_size,
                        &cloned_options,
                        &cancel,
                    );
                });
                
                // app = app.app_data(web::Data::new(connection)).service(
                //     web::scope("/api")
                //         .route("/shelly/archive", web::to(routes::archive_get_entries)),
                // );
            }
            Err(err) => return Err(err),
        }

    }

    println!("starting server at http://{}:{}", &bind.0, &bind.1);
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Logger::default().log_target("@"))
            .app_data(web::Data::new(options.clone()))
            .service(web::scope("/api").route("/shelly", web::to(routes::proxy_api_call)))
            .service(
                spa()
                    .index_file("./frontend_vue/shelly-plug-s/dist/index.html")
                    //.static_resources_mount("/static")
                    .static_resources_location("./frontend_vue/shelly-plug-s/dist")
                    .finish(),
            );
            if options.archive.is_some(){
                app = app.app_data(web::Data::new(routes::MemoryState{memory})).service(
                         web::scope("/archive")
                             .route("/shelly-plug", web::to(routes::archive_get_entries)));
            }
        app
    })
    .workers(1)
    .bind(bind)?
    .run()
    .await?;

    Ok(())
}
