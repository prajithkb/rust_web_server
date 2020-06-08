use crate::{job_command::JobCommand, job::Job};
use mpsc::Receiver;
use std::fmt::Debug;
use std::{
    sync::Arc,
    sync::{mpsc, Mutex},
    thread,
};

#[derive(Debug)]
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
    receiver: Arc<Mutex<Receiver<Job>>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let receiver_inside = receiver.clone();
        let thread = thread::spawn(move || loop {
            println!("Thread-{}, Waiting...", id);
            let job = receiver_inside.lock().unwrap().recv().unwrap();
            println!("Thread-{}, received Job {} {:?}", id, job.id, job.command);
            match job.command {
                JobCommand::RUN => {
                    (job.runnable)();
                    println!("Thread-{}, finished (RUN) Job {}", id, job.id);
                }
                JobCommand::STOP => {
                    break println!("Thread-{}, finished (STOP) Job {}", id, job.id);
                }
            }
        });
        Worker {
            id,
            thread,
            receiver,
        }
    }
}
