use bytes::Bytes;
use log::{debug, error, warn};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub(crate) fn run_tcp_client(
    addr: SocketAddr,
    tx: Sender<Bytes>,
    notifier: Arc<Notify>,
) -> JoinHandle<()> {
    debug!("+++TCP Client is starting");
    let token = CancellationToken::new();

    tokio::spawn(async move {
        loop {
            debug!("Subscribed to receiver");
            tokio::select! {
                result = TcpStream::connect(addr) => {
                    match result {
                        Ok(stream) => {
                            warn!("Gabriele Connection established");
                            let recv = tx.subscribe();
                            notifier.notify_waiters();
                            let _r = process_stream(stream, token.clone(), recv).await;
                            warn!("Connection closed");
                            break;
                        }
                        Err(_e) => {
                            error!("Error establishing TCP connection");
                        }
                    }
                    // wait 5s before connecting a new client
                    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                }

                _ = tokio::signal::ctrl_c() => {
                    warn!("Ctrl-C is captured for the tcp client");
                    break;
                }
            }
        }
    })
}

async fn process_stream(
    stream: TcpStream,
    token: CancellationToken,
    mut receiver: Receiver<Bytes>,
) {
    let (mut rx, mut tx) = stream.into_split();

    'outer: loop {
        if token.is_cancelled() {
            break;
        }

        tokio::select! {
            result = receiver.recv() => {
                match result {

                    Ok(chunk) => {
                        debug!("Client received bytes {chunk:?}");
                        for b in chunk {
                            if let Err(e) = tx.write_u8(b).await {
                               error!("Socket write error {e:?}");
                               break 'outer;
                            } else {
                                let reply = rx.read_u8().await.unwrap();
                                if reply != b {
                                    error!("Expected reply is {:02x?} but received {:02x?}", b, reply);
                                    break 'outer;
                                }
                            }
                        }
                    },

                    Err(RecvError::Lagged(n)) => {
                        error!("Receiver is lagged {n} messages");
                        break
                    }

                    Err(_) => break, // Channel is closed

                }
            }

            _ = token.cancelled() => {
               warn!("Cancelled TCP client");
                break;
            }

            _ = tokio::signal::ctrl_c() => {
                warn!("Ctrl-C received, exit reader");
                break;
            }
        }
    } // end loop
}
