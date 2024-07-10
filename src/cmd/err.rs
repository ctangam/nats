#[derive(Debug)]
pub struct Err {
    pub msg: String,
}

impl Err {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: String::from(msg),
        }
    }
}

impl Into<String> for Err {
    fn into(self) -> String {
        format!("-ERR {}\r\n", self.msg)
    }
}