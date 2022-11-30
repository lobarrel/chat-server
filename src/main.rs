use tokio::{net::TcpListener, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (mut socket, addr) = listener.accept().await.unwrap();
    let (reader, mut writer) = socket.split();

    let mut reader = BufReader::new(reader);
    
    let mut line = String::new();

    loop{
        let bytes_read = reader.read_line(&mut line).await.unwrap();

        writer.write_all(line.as_bytes()).await.unwrap();
    }

}