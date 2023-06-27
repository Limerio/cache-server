use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream};

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

    async fn read_and_write(&mut self, command: String) -> String {
        self.write(command).await;

        self.read().await
    }

    pub async fn del(&mut self, key: String) -> String {
        self.read_and_write(format!("DEL {}", key)).await
    }

    pub async fn get(&mut self, key: String) -> String {
        self.read_and_write(format!("GET {}", key)).await
    }

    pub async fn set(&mut self, key: String, value: String) -> String {
        self.read_and_write(format!("SET {} {}", key, value)).await
    }

    pub async fn all(&mut self) -> String {
        self.read_and_write("ALL".to_string()).await
    }

    pub async fn count(&mut self) -> String {
        self.read_and_write("COUNT".to_string()).await
    }

    pub async fn exists(&mut self, key: String) -> String {
        self.read_and_write(format!("EXISTS {}", key)).await
    }

    pub async fn flush(&mut self) -> String {
        self.read_and_write("FLUSH".to_string()).await
    }

    pub async fn rename(&mut self, old_key: String, new_key: String) -> String {
        self.read_and_write(format!("RENAME {} {}", old_key, new_key))
            .await
    }

    pub async fn ping(&mut self) -> String {
        self.read_and_write("PING".to_string()).await
    }
}
