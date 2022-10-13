use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream},
};

/// Accepts TcpStream and echos back all content until connection is closed
/// Protohackers: Solution 0
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

fn problem0() -> std::result::Result<(), std::io::Error> {
    const BIND: &str = "0.0.0.0:8080";
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
                println!("{}", format!("Failed to handle stream: {}", err));
            }
        }
    }
    Ok(())
}

fn problem1() -> std::io::Result<()> {
    const BIND: &str = "0.0.0.0:8081";
    let listener = TcpListener::bind(BIND).unwrap();
    println!("TCP Listener bound on {}. Listening...", BIND);
    for _ in listener.incoming() {

    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "0" => problem0(),
        "1" => problem1(),
        _ => Err(std::io::Error::new(
            ErrorKind::Other,
            format!("Unknown protohackers problem: {}", args[0]),
        )),
    }
}
