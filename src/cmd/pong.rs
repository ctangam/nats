pub struct Pong {

}

impl Pong {
    pub fn new() -> Self {
        Self {}
    }
}

impl Into<String> for Pong {
    fn into(self) -> String {
        format!("PONG\r\n")
    }
}