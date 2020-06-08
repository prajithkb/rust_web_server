use crate::job::Job;
use crate::worker::Worker;
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
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, r): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(r));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        let thread_pool = ThreadPool { workers, sender };
        println!("Initialized ThreadPool: {:#?}", thread_pool);
        thread_pool
    }
    pub fn execute(&self, f: Job) {
        let id = f.id.clone();
        self.sender.send(f).unwrap();
        println!("Sent Job {} to execute", id);
    }
}
