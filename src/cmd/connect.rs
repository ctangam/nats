#[derive(Debug)]
pub struct Connect {
}

impl Connect {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<String> for Connect {
    fn into(self) -> String {
        format!("CONNECT {}\r\n", "")
    }
}

impl TryFrom<String> for Connect {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        if value == "CONNECT {}\r\n" {
            Ok(Self::new())
        } else {
            Err(anyhow::Error::msg("Invalid CONNECT command"))
        }
    }
}