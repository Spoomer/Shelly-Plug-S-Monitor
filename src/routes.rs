use actix_web::{body::MessageBody, web, HttpResponse, Responder};
use crate::options::RunOptions;

pub async fn proxy_api_call(data: web::Data<RunOptions>) -> impl Responder {
    return match crate::shelly_api::get_from_shelly_plug_s(&data) {
        Ok(json) => HttpResponse::Ok().body(MessageBody::boxed(json)),
        Err(err) => {
            HttpResponse::BadRequest().body(MessageBody::boxed(err.to_string()))
        }
    }
}