mod connection;
pub use connection::Connection;

pub async fn del(port: String, key: String) -> String {
    connection::get_connection(port, format!("DEL {}", key)).await
}

pub async fn get(port: String, key: String) -> String {
    connection::get_connection(port, format!("GET {}", key)).await
}

pub async fn set(port: String, key: String, value: String) -> String {
    connection::get_connection(port, format!("SET {} {}", key, value)).await
}

pub async fn all(port: String) -> String {
    connection::get_connection(port, "ALL".to_string()).await
}
