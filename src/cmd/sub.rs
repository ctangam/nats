pub struct Sub {
    pub subject: String,
    pub queue_group: Option<String>,
    pub sid: String,
}

impl Sub {
    pub fn new(subject: String, queue_group: Option<String>, sid: String) -> Self {
        Self {
            subject,
            queue_group,
            sid,
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