use std::{
    collections::HashMap,
    pin::Pin,
    sync::{Arc, Mutex},
};

use anyhow::{Error, Result};
use bytes::{Buf, BytesMut};
use cmd::{Command, Err, Info, Msg, Ok, Pong};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    select,
    sync::broadcast,
};
use tokio_stream::{Stream, StreamExt, StreamMap};

pub mod cmd;

type Messages = Pin<Box<dyn Stream<Item = String> + Send>>;
type DB = Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:4222").await?;
    let db: DB = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream, db.clone()));
    }
}

async fn handle_connection(mut stream: TcpStream, db: DB) -> Result<(), Error> {
    let local_addr = stream.local_addr()?;
    let remote_addr = stream.peer_addr()?;
    let greeting: String = Info::new(
        &local_addr.ip().to_string(),
        local_addr.port(),
        &remote_addr.ip().to_string(),
    )
    .into();
    stream.write(greeting.as_bytes()).await?;

    let mut subscriptions: StreamMap<String, Messages> = StreamMap::new();
    loop {
        select! {
            Some((channel_name, msg)) = subscriptions.next() => {
                let msg: String = Msg::new(&channel_name, &channel_name, msg.len(), Some(&msg)).into();
                stream.write(msg.as_bytes()).await?;
            },
            rst = parse_cmd(&mut stream) => {
                dbg!(&rst);
                let cmd = match rst? { Some(cmd) => cmd, None => return Ok(()), };
                println!("Received command");
                handle_cmd(cmd, &mut stream, &db, &mut subscriptions).await?;
            },
        }
    }
}

async fn parse_cmd(stream: &mut TcpStream) -> Result<Option<Command>> {
    let mut buf = BytesMut::with_capacity(4 * 1024);
    loop {
        if 0 == stream.read_buf(&mut buf).await? {
            if buf.is_empty() {
                println!("Connection closed");
                return Ok(None);
            }

            println!("Connection reset by peer");
            return Err(Error::msg("Connection closed by peer"));
        }

        if let Some(cmd) = Command::parse(&buf[..])? {
            buf.clear();
            println!("Parsed command: {:?}", cmd);
            return Ok(Some(cmd));
        }

        match Command::parse(&buf[..]) {
            Ok(Some(cmd)) => {
                buf.clear();
                println!("Parsed command: {:?}", cmd);
                return Ok(Some(cmd));
            }
            Ok(None) => continue,
            Err(e) => {
                buf.clear();
                println!("Error parsing command: {:?}", e);
                return Ok(Some(Command::ERR(Err::new(&e.to_string()))));
            }
        }
    }
}

async fn handle_cmd(
    cmd: Command,
    stream: &mut TcpStream,
    db: &Arc<Mutex<HashMap<String, broadcast::Sender<String>>>>,
    subscriptions: &mut StreamMap<String, Pin<Box<dyn Stream<Item = String> + Send>>>,
) -> Result<(), Error> {
    Ok(match cmd {
        Command::CONNECT(_) => {
            let reply: String = Ok::new().into();
            stream.write(reply.as_bytes()).await?;
        }
        Command::PING(_) => {
            let reply: String = Pong::new().into();
            stream.write(reply.as_bytes()).await?;
        }
        Command::PUB(publish) => {
            let db = db.lock().unwrap();
            if let Some(tx) = db.get(&publish.subject) {
                let _ = tx.send(publish.payload.unwrap_or_default());
            }

            // let reply: String = Ok::new().into();
            // stream.write(reply.as_bytes()).await?;
        }
        Command::SUB(sub) => {
            let mut db = db.lock().unwrap();
            let tx = db
                .entry(sub.subject.clone())
                .or_insert_with(|| broadcast::channel(10).0);
            let mut rx = tx.subscribe();

            let rx = Box::pin(async_stream::stream! {
                loop {
                    if let Ok(msg) = rx.recv().await {
                        yield msg;
                    } else {
                        break;
                    }
                }
            });
            subscriptions.insert(sub.subject.clone(), rx);

            // let reply: String = Ok::new().into();
            // stream.write(reply.as_bytes()).await?;
        }
        Command::ERR(err) => {
            let reply: String = err.into();
            stream.write(reply.as_bytes()).await?;
        }
        _ => {
            println!("Invalid command");
        }
    })
}
