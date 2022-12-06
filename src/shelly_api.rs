use crate::options::RunOptions;

pub fn get_meter_status_from_shelly_plug_s(options: &RunOptions) -> Result<String, Box<dyn std::error::Error>> {
    let mut request_result = minreq::get("http://192.168.178.55/meter/0");
    if let Some(auth) = &options.authentication {
        request_result = request_result.with_header("Authorization", format!("Basic {}", auth));
    }
    let response = request_result.send()?;
    return match response.as_str() {
        Ok(json) => Ok(json.to_owned()),
        Err(err) => {
            Err(Box::new(err))
        }
    }
}