use serde_json::{Number, Value};

use crate::options::RunOptions;

pub fn get_meter_status_from_shelly_plug_s(
    options: &RunOptions,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = options.shelly_api_url.as_ref().ok_or("Shelly API URL nicht angegeben")?;
    let mut request_result = minreq::get(format!("http://{url}/meter/0"));
    if let Some(auth) = &options.authentication {
        request_result = request_result.with_header("Authorization", format!("Basic {}", auth));
    }
    let response = request_result.send()?;
    return match response.as_str() {
        Ok(json) => Ok(add_utc_offset(options, json)?.to_owned()),
        Err(err) => Err(Box::new(err)),
    };
}

fn add_utc_offset(options: &RunOptions, json: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut shelly_meter: Value = serde_json::from_str(json)?;
    let utc_offset: i32 = get_utc_offset_from_shelly_plug_s(options)?;
    shelly_meter["utcOffset"] = Value::Number(Number::from(utc_offset));
    Ok(serde_json::to_string(&shelly_meter)?)
}

pub fn get_utc_offset_from_shelly_plug_s(
    options: &RunOptions,
) -> Result<i32, Box<dyn std::error::Error>> {
    let url = options.shelly_api_url.as_ref().ok_or("Shelly API URL nicht angegeben")?;
    let mut request_result = minreq::get(format!("http://{url}/settings"));
    if let Some(auth) = &options.authentication {
        request_result = request_result.with_header("Authorization", format!("Basic {}", auth));
    }
    let response = request_result.send()?;
    let json = response.as_str()?;
    extract_offset_from_json(json)
}

fn extract_offset_from_json(json: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let json_obj: serde_json::Value = serde_json::from_str(json)?;
    match json_obj["tz_utc_offset"].as_i64() {
        Some(offset) => Ok(offset as i32),
        None => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Offset not in JSON or not parseable",
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::extract_offset_from_json;

    #[test]
    fn extract_offset_from_json_test() {
        let offset = extract_offset_from_json(SETTINGS_JSON);
        assert!(offset.is_ok());
        assert_eq!(3600, offset.unwrap());
    }
    const SETTINGS_JSON: &str = r#"{"device":{"type":"SHPLG-S","mac":"xx","hostname":"shellyplug-s-8850CD","num_outputs":1,"num_meters":1},"wifi_ap":{"enabled":false,"ssid":"shellyplug-s-8850CD","key":""},"wifi_sta":{"enabled":true,"ssid":"intrawebs","ipv4_method":"dhcp","ip":null,"gw":null,"mask":null,"dns":null},"wifi_sta1":{"enabled":false,"ssid":null,"ipv4_method":"dhcp","ip":null,"gw":null,"mask":null,"dns":null},"ap_roaming":{"enabled":false,"threshold":-70},"mqtt": {"enable":false,"server":"192.168.33.3:1883","user":"","id":"shellyplug-s-8850CD","reconnect_timeout_max":60.000000,"reconnect_timeout_min":2.000000,"clean_session":true,"keep_alive":60,"max_qos":0,"retain":false,"update_period":30},"coiot": {"enabled":true,"update_period":15,"peer":""},"sntp":{"server":"time.google.com","enabled":true},"login":{"enabled":true,"unprotected":false,"username":"admin"},"pin_code":"","name":"Shelly-Plug-S-1","fw":"20221027-101131/v1.12.1-ga9117d3","discoverable":true,"build_info":{"build_id":"20221027-101131/v1.12.1-ga9117d3","build_timestamp":"2022-10-27T10:11:31Z","build_version":"1.0"},"cloud":{"enabled":false,"connected":false},"timezone":"Europe/Berlin","lat":49.865711,"lng":8.626040,"tzautodetect":true,"tz_utc_offset":3600,"tz_dst":false,"tz_dst_auto":true,"time":"12:41","unixtime":1672227677,"led_status_disable":true,"debug_enable":false,"allow_cross_origin":false,"actions":{"active":false,"names":["btn_on_url","out_on_url","out_off_url"]},"hwinfo":{"hw_revision":"prod-190516","batch_id":1},"max_power":2500,"led_power_disable":false,"relays":[{"name":null,"appliance_type":"General","ison":true,"has_timer":false,"default_state":"on","auto_on":0.00,"auto_off":0.00,"schedule":true,"schedule_rules":["0030-01234-off","0600-01234-on"],"max_power":2500}],"eco_mode_enabled":true}"#;
}
