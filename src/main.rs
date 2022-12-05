use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_lab::web::spa;

use shelly_web::options::RunOptions;
use shelly_web::{archive, options, routes};
use std::{env, thread};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = get_run_options();
    let bind: (String, u16) = (String::from("127.0.0.1"), options.port);
    let cancel = false;

    if let Some(storage_size) = options.archive {
        let cloned_options = options.clone();
        match archive::init_archive(Some("./archive.db")) {
            Ok(connection) => {
                thread::spawn(move || {
                    archive::archive_service(
                        connection,
                        "./archive.db",
                        storage_size,
                        &cloned_options,
                        &cancel,
                    )
                });
            }
            Err(err) => return Err(err),
        }
    }

    println!("starting server at http://{}:{}", &bind.0, &bind.1);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default().log_target("@"))
            .app_data(web::Data::new(options.clone()))
            .service(web::scope("/api").route("/shelly", web::to(routes::proxy_api_call)))
            .service(
                spa()
                    .index_file("./frontendvue/shelly-plug-s/dist/index.html")
                    //.static_resources_mount("/static")
                    .static_resources_location("./frontendvue/shelly-plug-s/dist")
                    .finish(),
            )
    })
    .workers(1)
    .bind(bind)?
    .run().await?;

    Ok(())
}

fn get_run_options() -> RunOptions {
    let mut args = env::args();
    //ignore first argument: executable name
    args.next();
    let mut port: Option<String> = None;
    let mut archive: Option<String> = None;
    let mut authentication: Option<String> = None;

    //Env
    if let Ok(auth) = env::var("AUTH") {
        authentication = Some(auth);
    }
    // args overwrite Env
    while let Some(arg) = args.next() {
        if arg == "--port" || arg == "-p" {
            port = args.next();
        } else if arg == "--archive" || arg == "-a" {
            archive = args.next();
        } else if arg == "--auth" {
            authentication = args.next();
        }
    }
    return options::RunOptions::new(port, archive, authentication);
}
