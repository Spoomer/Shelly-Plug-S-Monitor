use actix_web::{
    body::MessageBody, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::web::spa;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = env::args();
    //ignore first argument: executable name
    args.next();
    let mut port: Option<String> = None;
    while let Some(arg) = args.next() {
        if arg == "-port" || arg == "-p" {
            port = args.next();
        }
    }

    let bind: (String, u16);
    if let Some(p) = port {
        if let Ok(port) = p.parse::<u16>() {
            bind = (String::from("127.0.0.1"), port);
        } else {
            bind = (String::from("127.0.0.1"), 8080);
        }
    } else {
        bind = (String::from("127.0.0.1"), 8080);
    }
    println!("starting server at http://{}:{}", &bind.0, &bind.1);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default().log_target("@"))
            .route("/api/shelly", web::to(proxy_api_call))
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
    .run()
    .await
}

async fn proxy_api_call() -> impl Responder {
    match get_from_shelly_plug_s() {
        Ok(json) => return HttpResponse::Ok().body(MessageBody::boxed(json.to_owned())),
        Err(err) => {
            return HttpResponse::BadRequest().body(MessageBody::boxed(err.to_string().to_owned()))
        }
    }
}

fn get_from_shelly_plug_s() -> Result<String, Box<dyn std::error::Error>> {
    let mut request_result = minreq::get("http://192.168.178.55/meter/0");
    if env::var("AUTH").is_ok() {
        request_result = request_result.with_header(
            "Authorization",
            format!("Basic {}", env::var("AUTH").unwrap()),
        );
    }
    let response = request_result.send()?;
    match response.as_str() {
        Ok(json) => return Ok(json.to_owned()),
        Err(err) => {
            return Err(Box::new(err));
        }
    }
}
