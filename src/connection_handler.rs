use std::borrow::Cow;
use std::io::prelude::*;
use std::{net::TcpStream, process::Command};

pub fn process(mut stream: TcpStream, id: i32) {
    let result = read_and_write(&mut stream, id);
    println!("Job {} Completed with result {:?}", id, result);
}
pub enum ExternalCommands {
    EXIT,
    TERM,
    PANIC,
    NONE,
}

fn read_and_write(stream: &mut TcpStream, id: i32) -> Result<String, String> {
    let mut term_enabled = false;
    loop {
        let prompt = if term_enabled { "Term" } else { "Welcome" };
        render_prompt(stream, prompt)?;
        println!("Job {}, blocking on read...", id);
        let mut buffer = [0; 512];
        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|e| format!("Error: {}", e))?;
        let mut characters: Vec<char> = Vec::new();
        for byte in &buffer[0..bytes_read] {
            characters.push(*byte as char);
        }
        println!("Received, {:?} bytes", characters);
        let message = String::from_utf8_lossy(&buffer[..]);
        match extract_command(&message) {
            ExternalCommands::EXIT => {
                respond_with(stream, "Bye!".to_string())?;
                return Ok("Success".to_string());
            }
            ExternalCommands::TERM => {
                if !term_enabled {
                    respond_with(stream, "Terminal enabled\n".to_string())?;
                    println!("Terminal enabled");
                    term_enabled = true;
                } else {
                    term_enabled = false;
                    respond_with(stream, "Terminal disabled\n".to_string())?;
                    println!("Terminal disabled");
                }
            }
            ExternalCommands::PANIC => {
                panic!("Panic");
            }
            _ => {
                if term_enabled {
                    let command = &message[0..bytes_read - 1];
                    println!("Running command [{}]", command);
                    respond_with(stream, run_shell_command(command))?;
                }
            }
        };
        respond_with(
            stream,
            format!("Received {} bytes, {:?}\n", bytes_read, characters),
        )?;
    }
}

macro_rules! invalid_command {
    ($command:expr, $error:expr) => {
        format!(
            "Invalid command \"{}\", obtained error [{}]\n",
            $command, $error
        )
    };
}

fn run_shell_command(message: &str) -> String {
    let tokens = extract_shell_comman_tokens(message);
    println!("Received {:?} for shell", tokens);
    if tokens.len() > 0 {
        let mut command = Command::new(tokens[0]);
        let output;
        if tokens.len() > 1 {
            output = command.args(&tokens[1..]).output();
        } else {
            output = command.output();
        }
        if output.is_ok() {
            let result = output.unwrap();
            if result.status.success() {
                shell_output(String::from_utf8_lossy(&result.stdout).to_string())
            } else {
                shell_output(String::from_utf8_lossy(&result.stderr).to_string())
            }
        } else {
            let error_message = output.unwrap_err().to_string();
            invalid_command!(message, error_message)
        }
    } else {
        invalid_command!(message, "Empty message")
    }
}

fn extract_shell_comman_tokens(message: &str) -> Vec<&str> {
    message.split(" ").collect()
}

fn shell_output(output: String) -> String {
    format!(
        "******** Output ********\n{}************************\n",
        output
    )
}

fn extract_command(message: &Cow<str>) -> ExternalCommands {
    match &message.to_uppercase()[0..4] {
        "EXIT" => ExternalCommands::EXIT,
        "TERM" => ExternalCommands::TERM,
        "BOMB" => ExternalCommands::PANIC,
        _ => ExternalCommands::NONE,
    }
}

fn render_prompt(stream: &mut TcpStream, prompt: &str) -> Result<(), String> {
    respond_with(stream, format!("{}:>", prompt))
}

pub fn respond_with(stream: &mut TcpStream, response: String) -> Result<(), String> {
    stream
        .write(response.as_bytes())
        .map_err(|e| format!("Error: {}", e))?;
    stream.flush().unwrap();
    Ok(())
}
