#[derive(Debug)]
pub struct Msg {
    pub subject: String,
    pub sid: String,
    pub bytes: usize,
    pub payload: Option<String>,
}

impl Msg {
    pub fn new(subject: &str, sid: &str, bytes: usize, payload: Option<&str>) -> Self {
        Self {
            subject: String::from(subject),
            sid: String::from(sid),
            bytes,
            payload: payload.map(String::from),
        }
    }
}

// MSG <subject> <sid> <#bytes>\r\n[payload]\r\n

impl Into<String> for Msg {
    fn into(self) -> String {
        if self.payload.is_some() {
            format!("MSG {} {} {}\r\n{}\r\n", self.subject, self.sid, self.bytes, self.payload.unwrap())
        } else {
            format!("MSG {} {} {}\r\n\r\n", self.subject, self.sid, self.bytes)
        }
    }
}