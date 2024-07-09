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

pub mod info;
pub use info::Info;

pub mod ok;
pub use ok::Ok;

pub mod msg;
pub use msg::Msg;

pub enum Command {
    OK(Ok),
    INFO(Info),
    CONNECT(Connect),
    PING(Ping),
    PONG(Pong),
    SUB(Sub),
    PUB(Publish),
    MSG(Msg),
}

impl Command {
    pub fn from(bytes: &[u8]) -> Option<Self> {
        let s = String::from_utf8_lossy(bytes);
        match s.split_whitespace().next() {
            Some("CONNECT") if s.ends_with("\r\n") => Some(Command::CONNECT(Connect::from(&*s))),
            Some("PING") if s.ends_with("\r\n") => Some(Command::PING(Ping::from(&*s))),
            Some("PONG") if s.ends_with("\r\n") => Some(Command::PONG(Pong::from(&*s))),
            Some("SUB") if s.ends_with("\r\n") => Some(Command::SUB(Sub::from(&*s))),
            Some("PUB") if s.matches("\r\n").count() == 2 => Some(Command::PUB(Publish::from(&*s))),
            _ => None,
        }
    }

    pub fn into(self) -> String {
        match self {
            Command::OK(ok) => ok.into(),
            Command::INFO(info) => info.into(),
            Command::CONNECT(connect) => connect.into(),
            Command::PING(ping) => ping.into(),
            Command::PONG(pong) => pong.into(),
            Command::SUB(sub) => sub.into(),
            Command::PUB(publish) => publish.into(),
            Command::MSG(msg) => msg.into(),
        }
    }
}
