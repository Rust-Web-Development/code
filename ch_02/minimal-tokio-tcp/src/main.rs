use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                match stream.read(&mut buf).await {
		            Ok(_) => {
			        stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").await.expect("Cannot write to stream");
			        stream.flush().await.expect("Cannot flush stream");
				return;
			     },
			     Err(e) => println!("{:?}", e),
			  }
            }
        });
     }
}
