use crate::job::Job;
use crate::{job_command::JobCommand, worker::Worker};
use mpsc::{Receiver, Sender};
use std::fmt::Debug;
use std::{
    sync::Arc,
    sync::{mpsc, Mutex},
};

pub type Runnable = Box<dyn FnOnce() + Send + 'static>;
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
    number_active_workers: u32,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0 && size < 10);
        let (sender, r): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(r));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        let thread_pool = ThreadPool {
            workers,
            sender,
            number_active_workers: 0,
        };
        println!("Initialized ThreadPool: {:#?}", thread_pool);
        thread_pool
    }
    pub fn execute(&mut self, f: Job) {
        match f.command {
            JobCommand::RUN => self.number_active_workers += 1,
            JobCommand::STOP => self.number_active_workers -=1
        }
        let id = f.id.clone();
        self.sender.send(f).unwrap();
        println!("Sent Job {} to execute", id);
        
    }
}
