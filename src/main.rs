use std::net::TcpListener;
use std::net::TcpStream;
use thread_pool::thread_pool::ThreadPool;
use thread_pool::task::Task;
use web_server::{
    connection_handler::{process, respond_with},
};
const SIZE: usize = 2;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut pool = ThreadPool::new(SIZE);
    let mut counter = 0;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut pool, &mut counter);
    }
}

fn handle_connection(stream: TcpStream, pool: &mut ThreadPool, counter: &mut i32) {
    *counter += 1;
    let peer_addr = stream.peer_addr();
    println!(
        "Accepted connection from {:?}, will create Job id {}, to handle the connection",
        peer_addr, counter
    );
    let id = counter.clone();
    let cloned_stream = stream.try_clone();
    let result = pool.execute(Task::new(
        Box::new(move || {
            process(stream, id);
        }),
        counter.to_string(),
    ));
    if result.is_err() {
        cloned_stream.map(|mut s| {
            respond_with(&mut s, format!("Error: {:?}", result.err())).unwrap_or_default();
        }).unwrap_or_default();
    }
}
