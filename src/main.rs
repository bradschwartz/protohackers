use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) -> std::result::Result<(), std::io::Error> {
    let mut buffer = [0; 10000];
    loop {
        let read_bytes_num = stream.read(&mut buffer)?;
        if read_bytes_num == 0 {
            return Ok(());
        }
        stream.write(&&buffer[..read_bytes_num])?;
        stream.flush()?;
        buffer = [0; 10000];
    }
}
fn main() -> std::io::Result<()> {
    const BIND: &str = "127.0.0.1:8080";
    let listener = TcpListener::bind(BIND).unwrap();
    println!("TCP Listener bound on {}. Listening...", BIND);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted stream {:?}", &stream);
                handle_connection(stream)?;
                println!("Handled stream successfully!");
            }
            Err(err) => {
                format!("Failed to handle stream: {}", err);
            }
        }
    }
    Ok(())
}