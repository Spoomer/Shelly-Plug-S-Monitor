use crate::{options::RunOptions, archive::Archive};
use actix_web::{body::MessageBody, web, HttpResponse, Responder};

pub async fn proxy_api_call(data: web::Data<RunOptions>) -> impl Responder {
    return match crate::shelly_api::get_meter_status_from_shelly_plug_s(&data) {
        Ok(json) => HttpResponse::Ok().body(MessageBody::boxed(json)),
        Err(err) => HttpResponse::BadRequest().body(MessageBody::boxed(err.to_string())),
    };
}

pub async fn archive_get_entries(
    memory_state : web::Data<MemoryState>,
    timespan_query: web::Query<TimespanQuery>,
) -> impl Responder {
    return match crate::archive::get_entries(memory_state.memory, timespan_query.from, timespan_query.to) {
        Ok(entries) => HttpResponse::Ok().body(MessageBody::boxed(serde_json::to_string::<Vec<Archive>>(&entries).unwrap())),
        Err(err) => HttpResponse::BadRequest().body(MessageBody::boxed(err.to_string())),
    };
}

#[derive(serde::Deserialize)]
pub struct TimespanQuery {
    pub from: u32,
    pub to: u32,
}

pub struct MemoryState {
    pub memory : bool
}