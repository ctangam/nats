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

pub mod err;
pub use err::Err;

#[derive(Debug)]
pub enum Command {
    OK(Ok),
    ERR(Err),
    INFO(Info),
    CONNECT(Connect),
    PING(Ping),
    PONG(Pong),
    SUB(Sub),
    PUB(Publish),
    MSG(Msg),
}

impl Command {
    pub fn parse(bytes: &[u8]) -> anyhow::Result<Option<Self>> {
        let s = String::from_utf8_lossy(bytes);
        match s.split_whitespace().next() {
            Some("CONNECT") => {
                if s.contains("\r\n") {
                    Ok(Some(Command::CONNECT(Connect::try_from(s.into_owned())?)))
                } else {
                    Ok(None)
                }
            },
            Some("PING") => {
                if s.contains("\r\n") {
                    Ok(Some(Command::PING(Ping::try_from(s.into_owned())?)))
                } else {
                    Ok(None)
                }
            }
            Some("PONG") => {
                if s.contains("\r\n") {
                    Ok(Some(Command::PONG(Pong::try_from(s.into_owned())?)))
                } else {
                    Ok(None)
                }
            }
            Some("SUB") => {
                if s.contains("\r\n") {
                    Ok(Some(Command::SUB(Sub::try_from(s.into_owned())?)))
                } else {
                    Ok(None)
                }
            }
            Some("PUB") => {
                if s.matches("\r\n").count() == 2 {
                    Ok(Some(Command::PUB(Publish::try_from(s.into_owned())?)))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
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
            Command::ERR(err) => err.into(),
        }
    }
}
