#[derive(Debug)]
pub struct Ok {
}

impl Ok {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<String> for Ok {
    fn into(self) -> String {
        format!("+OK\r\n")
    }
}