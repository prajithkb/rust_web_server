use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use web_server::Job;
use web_server::{Command, ThreadPool};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(2);
    let mut counter = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        counter += 1;
        let peer_addr = stream.peer_addr();
        println!(
            "Accepted connection from {:?}, will create Job id {}, to handle the connection",
            peer_addr, counter
        );
        let id = counter.clone();
        pool.execute(Job::new(
            Box::new(move || {
                handle_connection(stream, id);
            }),
            counter.to_string(),
            Command::RUNNABLE,
        ));
    }
}

fn handle_connection(mut stream: TcpStream, id: i32) {
    let result = read_and_write(&mut stream, id);
    println!("Job {} Completed with result {:?}", id, result);
}

fn read_and_write(stream: &mut TcpStream, id: i32) -> Result<String, String> {
    loop {
        println!("Job {}, blocking on read...", id);
        let mut buffer = [0; 512];
        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|e| format!("Error: {}", e))?;
        print!("Received [");
        for byte in &buffer[0..bytes_read] {
            print!("{:#?},", *byte as char);
        }
        println!("], {} bytes", bytes_read);
        let message = String::from_utf8_lossy(&buffer[..]);
        match &message.to_uppercase()[0..4] {
            "EXIT" => {
                return Ok("Success".to_string());
            }
            _ => {}
        };
        let response = format!("Received {} bytes!\n", bytes_read);
        stream
            .write(response.as_bytes())
            .map_err(|e| format!("Error: {}", e))?;
        stream.flush().unwrap();
    }
}
