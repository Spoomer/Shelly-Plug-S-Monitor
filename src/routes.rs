use crate::{aggreated_archive_data::EnergyData, options::RunOptions};
use actix_web::{
    http::header::{ContentDisposition, ContentType},
    web, HttpResponse, Responder,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn get_version() -> impl Responder {
    HttpResponse::Ok().body(VERSION)
}

pub async fn proxy_api_call(data: web::Data<RunOptions>) -> impl Responder {
    return match crate::shelly_api::get_meter_status_from_shelly_plug_s(&data) {
        Ok(json) => HttpResponse::Ok().body(json),
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    };
}

pub async fn archive_get_entries(
    memory_state: web::Data<MemoryState>,
    timespan_query: web::Query<TimespanQuery>,
) -> impl Responder {
    return match crate::archive_service::get_entries(
        memory_state.memory,
        1,
        timespan_query.from,
        timespan_query.to,
    ) {
        Ok(entries) => {
            HttpResponse::Ok().body(serde_json::to_string::<Vec<EnergyData>>(&entries).unwrap())
        }
        Err(err) => HttpResponse::BadRequest().body(err.to_string()),
    };
}

pub async fn archive_delete_entries(
    memory_state: web::Data<MemoryState>,
    plug_id_query: web::Query<PlugIdQuery>,
) -> impl Responder {
    return match crate::archive_service::delete_all_entries(
        memory_state.memory,
        plug_id_query.plug_id,
    ) {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    };
}

pub async fn archive_export_entries(
    memory_state: web::Data<MemoryState>,
    plug_id_query: web::Query<PlugIdQuery>,
) -> impl Responder {
    let filename = format!(
        "export-{}.csv",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    );
    return match crate::archive_service::export_all_entries(
        memory_state.memory,
        plug_id_query.plug_id,
    ) {
        Ok(export) => HttpResponse::Ok()
            .content_type(ContentType::octet_stream())
            .insert_header(ContentDisposition::attachment(filename))
            .body(export),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
}

#[derive(serde::Deserialize)]
pub struct TimespanQuery {
    pub from: u64,
    pub to: u64,
}

pub struct MemoryState {
    pub memory: bool,
}

#[derive(serde::Deserialize)]
pub struct PlugIdQuery {
    #[serde(rename(deserialize = "plugId"))]
    pub plug_id: u8,
}
