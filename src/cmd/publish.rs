pub struct Publish {
    pub subject: String,
    pub reply_to: Option<String>,
    pub size: usize,
    pub payload: Option<String>,
}

impl Publish {
    pub fn new(subject: String, reply_to: Option<String>, size: usize, payload: Option<String>) -> Self {
        Self {
            subject,
            reply_to,
            size,
            payload,
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