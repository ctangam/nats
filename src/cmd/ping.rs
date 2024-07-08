pub struct Ping {

}

impl Ping {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<String> for Ping {
    fn into(self) -> String {
        format!("PING\r\n")
    }
}

impl From<&str> for Ping {
    fn from(value: &str) -> Self {
        Self::new()
    }
}