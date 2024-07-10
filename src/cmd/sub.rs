use std::fmt::format;

#[derive(Debug)]
pub struct Sub {
    pub subject: String,
    pub queue_group: Option<String>,
    pub sid: String,
}

impl Sub {
    pub fn new(subject: &str, queue_group: Option<&str>, sid: &str) -> Self {
        Self {
            subject: String::from(subject),
            queue_group: queue_group.map(String::from),
            sid: String::from(sid),
        }
    }
}

// SUB <subject> [queue-group] <sid>\r\n

impl Into<String> for Sub {
    fn into(self) -> String {
        if self.queue_group.is_some() {
            format!("SUB {} {} {}\r\n", self.subject, self.queue_group.unwrap(), self.sid)
        } else {
            format!("SUB {} {}\r\n", self.subject, self.sid)
        }
    }
}

impl TryFrom<String> for Sub {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();
        dbg!(&parts);
        if parts.len() == 4 {
            Ok(Self::new(parts[1], Some(parts[2]), parts[3]))
        } else if parts.len() == 3 {
            Ok(Self::new(parts[1], None, parts[2]))
        } else {
            Err(anyhow::Error::msg(format!("Invalid SUB command: {}", value)))
        }
    }
}

impl From<&str> for Sub {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        if parts.len() == 5 {
            Self::new(parts[1], Some(parts[2]), parts[3])
        } else {
            Self::new(parts[1], None, parts[2])
        }
    }
}