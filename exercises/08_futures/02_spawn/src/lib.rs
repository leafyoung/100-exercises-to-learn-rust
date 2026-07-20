use tokio::{net::TcpListener, task::JoinError};

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.

async fn echo_loop(mut listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        match listener.accept().await {
            Ok((mut socket, _)) => tokio::spawn(async move {
                let (mut reader, mut writer) = socket.split();
                let _ = tokio::io::copy(&mut reader, &mut writer).await;
            }),
            Err(_) => continue,
        };
    }
}

pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    let (h1, h2) = tokio::join!(
        tokio::spawn(echo_loop(first)),
        tokio::spawn(echo_loop(second)),
    );
    h1?;
    h2?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in &requests {
            for addr in [first_addr, second_addr] {
                let request = request.clone();
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
