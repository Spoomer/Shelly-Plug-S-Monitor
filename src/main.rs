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

    let mut bind = ("127.0.0.1", 8080);
    if let Some(p) = port {
        if let Ok(port) = p.parse::<u16>() {
            bind.1 = port;
        }
    }
    println!("starting server at http://{}:{}", &bind.0, &bind.1);
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default().log_target("@"))
            .route("/api/shelly", web::to(proxy_api_call))
            .service(
                spa()
                    .index_file("./frontend/build/index.html")
                    .static_resources_mount("/static")
                    .static_resources_location("frontend/build/static/")
                    .finish(),
            )
    })
    .workers(1)
    .bind(bind)?
    .run()
    .await
}
async fn proxy_api_call() -> impl Responder {
    let mut request_result = minreq::get("http://192.168.178.55/meter/0");
    if env::var("AUTH").is_ok() {
        request_result = request_result.with_header(
            "Authorization",
            format!("Basic {}", env::var("AUTH").unwrap()),
        )
    }
    if let Ok(response) = request_result.send() {
        match response.as_str() {
            Ok(json) => return HttpResponse::Ok().body(MessageBody::boxed(json.to_owned())),
            Err(err) => {
                return HttpResponse::BadRequest()
                    .body(MessageBody::boxed(err.to_string().to_owned()))
            }
        }
    } else {
        return HttpResponse::BadRequest().finish();
    }
}
