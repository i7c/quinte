use super::{notmuch, Error, Result};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    io,
    net::SocketAddr,
    sync::Arc,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, channel, Receiver, Sender},
};
use tokio_tungstenite::{self, accept_async, WebSocketStream};
use tungstenite::protocol::Message;

pub async fn listen(db: Arc<notmuch::NotmuchDb>) -> io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:42337").await?;

    info!("Waiting for connections");

    while let Ok((stream, remote_address)) = listener.accept().await {
        let db = db.clone();
        info!("Connection attempt: {}", remote_address);
        tokio::spawn(async move {
            let res = accept_connection(stream, remote_address, db).await;
            if let Err(e) = res {
                error!("Connection from {} failed: {:?}", remote_address, e);
            }
        });
    }
    Ok(())
}

async fn accept_connection(
    tcp_stream: TcpStream,
    remote_address: SocketAddr,
    db: Arc<notmuch::NotmuchDb>,
) -> Result<()> {
    // Gives us a sink and stream to talk over the websocket
    let (mut wstx, wsrx) = accept_async(tcp_stream).await?.split();
    info!("Established connection from {}", remote_address);

    // Say hello to our peer
    wstx.send(
        QuinteFrame {
            cid: "quinte/server/1".to_owned(),
            payload: QuintePayload::ServerGreeting {
                server: "quinte server".to_owned(),
                protocol: 0,
            },
        }
        .into(),
    )
    .await?;

    let (tx, rx) = channel::<QuinteFrame>(3);

    tokio::spawn(sender_task(wstx, rx));
    receiver_task(wsrx, tx, db, remote_address).await
}

async fn sender_task(
    mut wstx: SplitSink<WebSocketStream<TcpStream>, Message>,
    mut rx: Receiver<QuinteFrame>,
) {
    while let Some(frame) = rx.next().await {
        let res = wstx.send(frame.into()).await;
        if let Err(e) = res {
            error!("Failed to send frame: {:?}", e);
        }
    }
}

async fn receiver_task(
    mut wsrx: SplitStream<WebSocketStream<TcpStream>>,
    tx: Sender<QuinteFrame>,
    db: Arc<notmuch::NotmuchDb>,
    remote_address: SocketAddr,
) -> Result<()> {
    while let Some(result) = wsrx.next().await {
        let mut tx = tx.clone();
        let db = db.clone();
        match result {
            Ok(message) => match message {
                Message::Text(payload) => {
                    let parse_result: Result<QuinteFrame> = payload.try_into();

                    match parse_result {
                        Ok(frame) => {
                            tokio::spawn(async move {
                                let cid = frame.cid.to_owned();
                                let process_result = process_frame(frame, tx.clone(), db).await;

                                if let Err(e) = process_result {
                                    tx.send(QuinteFrame::error(
                                        cid,
                                        format!("Error when processing frame: {:?}", e),
                                    ))
                                    .await
                                    .unwrap()
                                }
                            });
                        }
                        Err(e) => tx
                            .send(QuinteFrame::error(
                                "quinte/server/no_correlation_id".to_owned(),
                                format!("Failed to parse message: {:?}", e),
                            ))
                            .await
                            .unwrap(),
                    }
                }
                Message::Close(_) => info!("Client indicates connection close"),
                _ => error!("Client {} sent unhandled message format", remote_address),
            },
            Err(err) => {
                error!("Couldn't receive data: {}", err);
            }
        }
    }
    info!("Closing connection from {}", remote_address);
    Ok(())
}

async fn process_frame(
    frame: QuinteFrame,
    mut response_channel: Sender<QuinteFrame>,
    db: Arc<notmuch::NotmuchDb>,
) -> Result<()> {
    match frame.payload {
        QuintePayload::Ping => {
            response_channel
                .send(QuinteFrame {
                    cid: frame.cid,
                    payload: QuintePayload::Pong,
                })
                .await?;
            Ok(())
        }

        QuintePayload::MailSearch(query) => {
            let mails = mail_search(query, db)?;
            response_channel
                .send(QuinteFrame {
                    cid: frame.cid,
                    payload: QuintePayload::MailList(mails),
                })
                .await?;
            Ok(())
        }

        QuintePayload::LoadMail(message_id) => {
            let mail = db.load_mail(&message_id)?;
            response_channel
                .send(QuinteFrame {
                    cid: frame.cid,
                    payload: QuintePayload::Mail(mail),
                })
                .await?;
            Ok(())
        }

        _ => Err(Error::UnknownPayload),
    }
}

fn mail_search(
    query: String,
    db: Arc<notmuch::NotmuchDb>,
) -> Result<Vec<notmuch::message::Message>> {
    let mail_iterator = db.search(&query)?;

    let mut mails = vec![];
    for mail in mail_iterator {
        mails.push(mail);
    }
    Ok(mails)
}

#[derive(Debug, Serialize, Deserialize)]
struct QuinteFrame {
    cid: String,
    payload: QuintePayload,
}

impl QuinteFrame {
    fn error(cid: String, message: String) -> Self {
        QuinteFrame {
            cid,
            payload: QuintePayload::Error(message),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum QuintePayload {
    Error(String),
    LoadMail(String),
    Mail(String),
    MailSearch(String),
    MailList(Vec<notmuch::message::Message>),
    Ping,
    Pong,
    ServerGreeting { server: String, protocol: u32 },
}

impl From<tungstenite::error::Error> for Error {
    fn from(e: tungstenite::error::Error) -> Error {
        Error::WebSocket(e.to_string())
    }
}

impl<T> From<mpsc::error::SendError<T>> for Error {
    fn from(_: mpsc::error::SendError<T>) -> Self {
        Error::Internal("Sender task has hung up unexpectedly.")
    }
}

impl From<QuinteFrame> for Message {
    fn from(f: QuinteFrame) -> Self {
        Message::Text(serde_json::to_string(&f).unwrap())
    }
}

impl TryFrom<String> for QuinteFrame {
    type Error = Error;

    fn try_from(s: String) -> Result<QuinteFrame> {
        serde_json::from_str(&s).map_err(|e| Error::FrameParse(e.to_string()))
    }
}
