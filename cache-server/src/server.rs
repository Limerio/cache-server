use crate::Db;
use bytes::Bytes;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle_connection(stream: &mut TcpStream, db: &mut Db) -> io::Result<()> {
    loop {
        let mut buf = [0; 512];
        let bytes_read = stream.read(&mut buf).await?;
        if bytes_read == 0 {
            break;
        }

        let s = String::from_utf8(buf.to_vec());
        if s.is_err() {
            stream.write_all("+UNKNOWN\r\n".as_bytes()).await?;
        }

        let filtered_char = s.unwrap();
        let filtered_parts: Vec<&str> = filtered_char.trim_end_matches('\0').split(' ').collect();

        if filtered_parts.is_empty() {
            continue;
        }

        match filtered_parts[0] {
            "SET" => {
                if filtered_parts.len() >= 3 {
                    let key = String::from(filtered_parts[1]);
                    let value = Bytes::from(filtered_parts[2].to_owned());
                    db.set(key, value);
                    stream.write_all("OK\r\n".as_bytes()).await?;
                } else {
                    stream
                        .write_all("INVALID COMMAND SET\r\n".as_bytes())
                        .await?;
                    continue;
                }
            }
            "GET" => {
                if filtered_parts.len() >= 2 {
                    let key = String::from(filtered_parts[1]);

                    stream
                        .write_all(format!("{:}\r\n", db.get(key)).as_bytes())
                        .await?;
                } else {
                    stream
                        .write_all("INVALID COMMAND GET\r\n".as_bytes())
                        .await?;
                    continue;
                }
            }
            "DEL" => {
                if filtered_parts.len() >= 2 {
                    let key = String::from(filtered_parts[1]);
                    db.del(key);
                    stream.write_all("OK\r\n".as_bytes()).await?;
                } else {
                    stream
                        .write_all("INVALID COMMAND DEL\r\n".as_bytes())
                        .await?;
                    continue;
                }
            }
            "ALL" => stream.write_all(db.all().as_bytes()).await?,
            "PING" => {
                stream.write_all("PONG".as_bytes()).await?;
            }
            _ => {
                stream.write_all("UNKNOWN COMMAND\r\n".as_bytes()).await?;
                continue;
            }
        }
    }
    Ok(())
}
