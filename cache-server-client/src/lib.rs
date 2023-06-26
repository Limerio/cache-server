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

pub async fn count(port: String) -> String {
    connection::get_connection(port, "COUNT".to_string()).await
}

pub async fn exists(port: String, key: String) -> String {
    connection::get_connection(port, format!("EXISTS {}", key)).await
}

pub async fn flush(port: String) -> String {
    connection::get_connection(port, "FLUSH".to_string()).await
}

pub async fn rename(port: String, old_key: String, new_key: String) -> String {
    connection::get_connection(port, format!("RENAME {} {}", old_key, new_key)).await
}

pub async fn ping(port: String) -> String {
    connection::get_connection(port, "PING".to_string()).await
}
