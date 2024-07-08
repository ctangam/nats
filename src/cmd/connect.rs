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