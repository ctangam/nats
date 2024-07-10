#[derive(Debug)]
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

impl TryFrom<String> for Ping {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        if value == "PING\r\n" {
            Ok(Self::new())
        } else {
            Err(anyhow::Error::msg("Invalid PING command"))
        }
    }
}

impl From<&str> for Ping {
    fn from(value: &str) -> Self {
        Self::new()
    }
}

