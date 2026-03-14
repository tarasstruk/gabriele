use bytes::Bytes;
use log::{debug, error, warn};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::Receiver;
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

pub(crate) fn run_tcp_client(
    addr: SocketAddr,
    rx: Receiver<Bytes>,
    notifier: Arc<Notify>,
    token: CancellationToken,
) -> JoinHandle<()> {
    debug!("+++TCP Client is starting");

    tokio::spawn(async move {
        loop {
            debug!("Subscribed to receiver");
            tokio::select! {
                result = TcpStream::connect(addr) => {
                    match result {
                        Ok(stream) => {
                            warn!("Gabriele Connection established");
                            notifier.notify_waiters();
                            process_stream(stream, rx).await;
                            warn!("TCP Connection closed");
                            break;
                        }
                        Err(_e) => {
                            error!("Error establishing TCP connection");
                            if token.is_cancelled() {
                                warn!("token is cancelled");
                                break;
                            }
                            // wait 5s before connecting a new client
                            tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
                        }
                    }
                }

                _ = token.cancelled() => {
                   warn!("Cancelled loop");
                    break;
                }

                _ = tokio::signal::ctrl_c() => {
                    warn!("Ctrl-C is captured for the tcp client");
                    break;
                }
            }
        }
    })
}

async fn process_stream(stream: TcpStream, mut receiver: Receiver<Bytes>) {
    let (mut rx, mut tx) = stream.into_split();

    'outer: loop {
        tokio::select! {
            result = receiver.recv() => {
                match result {
                    Ok(chunk) => {
                        debug!("Client received bytes {:02x}", chunk);
                        // this is regarded as inner loop
                        for byte in chunk {
                            if let Err(e) = tx.write_u8(byte).await {
                               error!("Socket write error {e:?}");
                               break 'outer;
                            } else {
                                match rx.read_u8().await {
                                    Ok(reply) if (reply != byte) => {
                                        error!("Expected reply is {:02x} but received {:02x}", byte, reply);
                                        break 'outer;
                                    }
                                    Err(e) => {
                                        error!("Socket read error {e:?}");
                                        break 'outer;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    },

                    Err(RecvError::Lagged(n)) => {
                        error!("Receiver is lagged {n} messages");
                        break
                    }

                    Err(RecvError::Closed) => {
                        warn!("Channel is closed");
                        break
                    }

                }
            }

            _ = tokio::signal::ctrl_c() => {
                warn!("Ctrl-C received, exit reader");
                break;
            }
        }
    } // end loop
}
