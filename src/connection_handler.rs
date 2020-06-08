use std::borrow::Cow;
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;

pub fn handle_connection(mut stream: TcpStream, id: i32) {
    let result = read_and_write(&mut stream, id);
    println!("Job {} Completed with result {:?}", id, result);
}
pub enum ExternalCommands {
    EXIT,
    NONE
}

fn read_and_write(stream: &mut TcpStream, id: i32) -> Result<String, String> {
    loop {
        render_prompt(stream)?;
        println!("Job {}, blocking on read...", id);
        let mut buffer = [0; 512];
        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|e| format!("Error: {}", e))?;
        let mut characters : Vec<char> = Vec::new();
        for byte in &buffer[0..bytes_read] {
            characters.push(*byte as char);
        }
        println!("Received, {:?} bytes", characters);
        let message = String::from_utf8_lossy(&buffer[..]);
        match extract_command(message) {
            ExternalCommands::EXIT => {
                respond_with(stream, "Bye!".to_string())?;
                return Ok("Success".to_string());
            },
            _ => ()
        };
        respond_with(stream, format!("Received {} bytes, {:?}\n", bytes_read, characters))?;
    }
}


fn extract_command(message: Cow<str>) -> ExternalCommands {
    match &message.to_uppercase()[0..4] {
        "EXIT" => ExternalCommands::EXIT,
        _ => ExternalCommands::NONE
    }
}

fn render_prompt(stream: &mut TcpStream) -> Result<(), String> {
    respond_with(stream, "Enter something:>".to_string())
}

fn respond_with(stream: &mut TcpStream, response: String) -> Result<(), String> {
    stream
        .write(response.as_bytes())
        .map_err(|e| format!("Error: {}", e))?;
    stream.flush().unwrap();
    Ok(())
}
