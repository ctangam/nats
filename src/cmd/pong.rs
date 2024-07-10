#[derive(Debug)]
pub struct Pong {

}

impl Pong {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<String> for Pong {
    fn into(self) -> String {
        format!("PONG\r\n")
    }
}

impl TryFrom<String> for Pong {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        if value == "PONG\r\n" {
            Ok(Self::new())
        } else {
            Err(anyhow::Error::msg("Invalid PONG command"))
        }
    }
}

impl From<&str> for Pong {
    fn from(value: &str) -> Self {
        Self::new()
    }
}