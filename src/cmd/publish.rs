#[derive(Debug)]
pub struct Publish {
    pub subject: String,
    pub reply_to: Option<String>,
    pub size: usize,
    pub payload: Option<String>,
}

impl Publish {
    pub fn new(subject: &str, reply_to: Option<&str>, size: usize, payload: Option<&str>) -> Self {
        Self {
            subject: String::from(subject),
            reply_to: reply_to.map(String::from),
            size,
            payload: payload.map(String::from),
        }
    }
}

// PUB <subject> [reply-to] <#bytes>\r\n[payload]\r\n

impl Into<String> for Publish {
    fn into(self) -> String {
        if self.reply_to.is_some() {
            format!("PUB {} {} {}\r\n{}\r\n", self.subject, self.reply_to.unwrap(), self.size, self.payload.unwrap_or_default())
        } else {
            format!("PUB {} {}\r\n{}\r\n", self.subject, self.size, self.payload.unwrap_or_default())
        }
    }
}

impl TryFrom<String> for Publish {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split("\r\n").collect();
        dbg!(&parts);
        let meta: Vec<&str> = parts[0].split_whitespace().collect();
        dbg!(&meta);
        let payload = if parts[1].is_empty() {
            None
        } else {
            Some(parts[1])
        };
        if meta.len() == 4 {
            Ok(Self::new(meta[1], Some(meta[2]), meta[3].parse().unwrap(), payload))
        } else if meta.len() == 3 {
            Ok(Self::new(meta[1], None, meta[2].parse().unwrap(), payload))
        } else {
            Err(anyhow::Error::msg(format!("Invalid PUB command: {}", value)))
        }
    }
}

impl From<&str> for Publish {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split("\r\n").collect();
        let meta: Vec<&str> = parts[0].split_whitespace().collect();
        let payload = if parts[1].is_empty() {
            None
        } else {
            Some(parts[1])
        };
        if meta.len() == 4 {
            Self::new(meta[1], Some(meta[2]), meta[3].parse().unwrap(), payload)
        } else {
            Self::new(meta[1], None, meta[2].parse().unwrap(), payload)

        }
    }
}