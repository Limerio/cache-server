use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

pub async fn get_connection(port: String, command: String) -> String {
    let mut connection = Connection::new(port).await;
    connection.write(command).await;

    connection.read().await
}

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub async fn new(port: String) -> Connection {
        Self {
            stream: TcpStream::connect(format!("127.0.0.1:{}", port))
                .await
                .expect("Unable to connect to the database"),
        }
    }

    pub async fn write(&mut self, command: String) {
        self.stream.write_all(command.as_bytes()).await.unwrap()
    }

    pub async fn read(&mut self) -> String {
        let mut buffer = [0; 512];
        let bytes_read = self.stream.read(&mut buffer).await.unwrap();

        String::from_utf8_lossy(&buffer[..bytes_read]).to_string()
    }
}
