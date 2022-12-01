use actix_web::{body::MessageBody, web, HttpResponse, Responder};

pub struct RunOptions {
    pub port: u16,
    pub archive: Option<usize>,
    pub authorization: Option<String>,
}

impl RunOptions {
    pub fn new(
        port: Option<String>,
        archive: Option<String>,
        authorization: Option<String>,
    ) -> Self {
        let mut archive_parse_result: Option<usize> = None;
        if let Some(a) = archive {
            if let Ok(archive_u16) = a.parse::<usize>() {
                archive_parse_result = Some(archive_u16)
            }
        }

        if let Some(p) = port {
            if let Ok(port) = p.parse::<u16>() {
                return Self {
                    port,
                    archive: archive_parse_result,
                    authorization,
                };
            } else {
                return Self {
                    port: 8080,
                    archive: archive_parse_result,
                    authorization,
                };
            }
        }
        Self {
            port: 8080,
            archive: archive_parse_result,
            authorization,
        }
    }
}
impl Clone for RunOptions {
    fn clone(&self) -> Self {
        Self {
            port: self.port.clone(),
            archive: self.archive.clone(),
            authorization: self.authorization.clone(),
        }
    }
}

pub async fn proxy_api_call(data: web::Data<RunOptions>) -> impl Responder {
    match get_from_shelly_plug_s(&data) {
        Ok(json) => return HttpResponse::Ok().body(MessageBody::boxed(json.to_owned())),
        Err(err) => {
            return HttpResponse::BadRequest().body(MessageBody::boxed(err.to_string().to_owned()))
        }
    }
}

pub fn get_from_shelly_plug_s(options: &RunOptions) -> Result<String, Box<dyn std::error::Error>> {
    let mut request_result = minreq::get("http://192.168.178.55/meter/0");
    if let Some(auth) = &options.authorization {
        request_result = request_result.with_header("Authorization", format!("Basic {}", auth));
    }
    let response = request_result.send()?;
    match response.as_str() {
        Ok(json) => return Ok(json.to_owned()),
        Err(err) => {
            return Err(Box::new(err));
        }
    }
}

pub fn archive_data(storage_size: usize, cancel: &bool){

    let conn = rusqlite::Connection::open("./archive.db").unwrap();
    while !cancel {
        conn.execute("",()).expect("Failed");
    };
}
