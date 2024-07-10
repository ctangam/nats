#[derive(Debug)]
pub struct UnSub {
    pub sid: String,
}

impl UnSub {
    pub fn new(sid: &str) -> Self {
        Self {
            sid: String::from(sid),
        }
    }
}

impl Into<String> for UnSub {
    fn into(self) -> String {
        format!("UNSUB {}\r\n", self.sid)
    }
}

impl TryFrom<String> for UnSub {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() == 2 {
            Ok(Self::new(parts[1]))
        } else {
            Err(anyhow::Error::msg(format!("Invalid UNSUB command: {}", value)))
        }
    }
}