
pub mod connect;
pub use connect::Connect;

pub mod ping;
pub use ping::Ping;

pub mod pong;
pub use pong::Pong;

pub mod publish;
pub use publish::Publish;

pub mod sub;
pub use sub::Sub;


pub enum Command {
    CONNECT(Connect),
    PING(Ping),
    PONG(Pong),
    SUB(Sub),
    PUB(Publish),
}

impl Command {
    pub fn from(bytes: &[u8]) -> Self {
        let s = String::from_utf8_lossy(bytes);
        match s.split_whitespace().next() {
            Some("CONNECT") => Command::CONNECT(Connect::from(&*s)),
            Some("PING") => Command::PING(Ping::from(&*s)),
            Some("PONG") => Command::PONG(Pong::from(&*s)),
            Some("SUB") => Command::SUB(Sub::from(&*s)),
            Some("PUB") => Command::PUB(Publish::from(&*s)),
            _ => unimplemented!()
        }
    }
}