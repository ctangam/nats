use std::fmt::Formatter;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Info {
    pub host: String,
    pub port: u16,
    pub client_ip: String,
}

impl Info {
    pub fn new(host: &str, port: u16, client_ip: &str) -> Self {
        Self {
            host: String::from(host),
            port,
            client_ip: String::from(client_ip),
        }
    }
}


impl Into<String> for Info {
    fn into(self) -> String {
        format!("INFO {}\r\n", serde_json::to_string(&self).unwrap())
    }
}


