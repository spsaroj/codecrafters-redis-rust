#![allow(unused_imports)]
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    loop{
        let stream = listener.accept().await.unwrap();
        match stream {
            Ok(mut stream, _) => {
                tokio::spawn(async move {
                    let mut buf = [0;512];
                    loop{
                        let read_count = stream.read(&mut buf).unwrap();
                        if(read_count==0){
                            break;
                        }
                        stream.write(b"+PONG\r\n").unwrap();
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
