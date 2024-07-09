use std::{collections::HashMap, pin::Pin, sync::{Arc, Mutex}};

use anyhow::{Result, Error};
use bytes::{Buf, BytesMut};
use cmd::{Command, Info, Msg, Ok, Pong};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, select, sync::broadcast};
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
    let mut subscriptions: StreamMap<String, Messages> = StreamMap::new();

    let local_addr = stream.local_addr()?;
    let remote_addr = stream.peer_addr()?;
    let greeting: String = Info::new(&local_addr.ip().to_string(), local_addr.port(), &remote_addr.ip().to_string()).into();
    stream.write(greeting.as_bytes()).await?;
    select! {
        Some((channel_name, msg)) = subscriptions.next() => {},
        _ = handle_cmd(stream, db, &mut subscriptions) => {},
    }
    Ok(())
}


async fn handle_cmd(mut stream: TcpStream, db: DB, subscriptions: &mut StreamMap<String, Messages>) -> Result<(), Error> {
    let mut buf = BytesMut::with_capacity(4 * 1024);
    loop {
        let n = stream.read_buf(&mut buf).await?;
        if n == 0 {
            println!("Connection closed by peer");
            break;
        }

        if let Some(cmd) = Command::from(&buf[..]) {
            match cmd {
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
                    let tx = db.get(&publish.subject).unwrap();
                    let _ = tx.send(publish.payload.unwrap_or_default());
                }
                Command::SUB(sub) => {
                    let mut db = db.lock().unwrap();
                    let tx = db.entry(sub.subject.clone()).or_insert_with(|| broadcast::channel(10).0);
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
                }
                _ => {
                    println!("Invalid command");
                }
            }
            buf.clear();
        } else {
            println!("Invalid command: {}", String::from_utf8_lossy(&buf));
            continue;
        }
    }
    Ok(())
}
