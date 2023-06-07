use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn get_response(listener: &mut TcpStream) -> String {
    let mut buffer = [0; 512];
    let bytes_read = listener.read(&mut buffer).await.unwrap();

    return String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
}
