use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct RunOptions {
    pub host: Option<String>,
    pub port: u16,
    pub archive: Option<usize>,
    pub authentication: Option<String>,
    #[serde(default, rename = "shellyApiUrl")]
    pub shelly_api_url: Option<String>,
}

impl RunOptions {
    pub fn add_port(&mut self, port: Option<String>) {
        if let Some(p) = port {
            if let Ok(port) = p.parse::<u16>() {
                self.port = port;
            }
        }
    }
    pub fn add_archive(&mut self, archive: Option<String>) {
        if let Some(a) = archive {
            if let Ok(archive_u16) = a.parse::<usize>() {
                self.archive = Some(archive_u16)
            }
        }
    }
}
impl Clone for RunOptions {
    fn clone(&self) -> Self {
        Self {
            host: self.host.clone(),
            port: self.port,
            archive: self.archive,
            authentication: self.authentication.clone(),
            shelly_api_url: self.shelly_api_url.clone(),
        }
    }
}

pub fn get_run_options() -> RunOptions {
    let mut options: RunOptions = RunOptions {
        port: 8080,
        archive: None,
        authentication: None,
        shelly_api_url: None,
        host: None,
    };
    if let Some(config) = read_config() {
        options = config;
    }

    let mut args = env::args();
    //ignore first argument: executable name
    args.next();

    //Env
    if let Ok(auth) = env::var("AUTH") {
        options.authentication = Some(auth);
    }
    // args overwrite Env
    while let Some(arg) = args.next() {
        if arg == "--port" || arg == "-p" {
            options.add_port(args.next());
        } else if arg == "--archive" || arg == "-a" {
            options.add_archive(args.next());
        } else if arg == "--auth" {
            options.authentication = args.next();
        }
        else if arg == "--shelly-api-url" {
            options.shelly_api_url = args.next();
        }
        else if arg == "--host" {
            options.host = args.next();
        }
    }
    options
}

fn read_config() -> Option<RunOptions> {
    let config_filepath = std::path::Path::new("config.json");
    if config_filepath.exists() {
        if let Ok(config) = std::fs::read_to_string(config_filepath) {
            if let Ok(options) = serde_json::from_str::<RunOptions>(&config) {
                return Some(options);
            }
        }
    }
    None
}
