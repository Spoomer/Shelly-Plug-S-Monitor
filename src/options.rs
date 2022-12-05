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
            return if let Ok(port) = p.parse::<u16>() {
                Self {
                    port,
                    archive: archive_parse_result,
                    authorization,
                }
            } else {
                Self {
                    port: 8080,
                    archive: archive_parse_result,
                    authorization,
                }
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
            port: self.port,
            archive: self.archive,
            authorization: self.authorization.clone(),
        }
    }
}