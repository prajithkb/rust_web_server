use std::net::TcpListener;
use web_server::{command::Command, job::Job, thread_pool::ThreadPool, connection_handler::handle_connection};
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