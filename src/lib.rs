use std::{sync::{Mutex, mpsc}, thread, sync::Arc};
use mpsc::{Receiver, Sender};

type Runnable = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
}

pub struct Job {
    runnable: Runnable,
    id: String,
    command: Command
}

#[derive(Debug)]
pub enum Command {
    RUNNABLE,
    INTERRUPT
}

impl Job {
    pub fn new(runnable :Runnable, id :String, command: Command) -> Job {
        Job{
            runnable,
            id,
            command
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, r):(Sender<Job>, Receiver<Job>) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(r));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute(&self, f: Job) {
        let id = f.id.clone();
        self.sender.send(f).unwrap();
        println!("Sent Job {} to execute", id);
    }
}
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
    receiver: Arc<Mutex<Receiver<Job>>>

}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let receiver_inside = receiver.clone();
        let thread = thread::spawn(move || {
            loop {
                println!("Thread-{}, Waiting...", id);
                let job = receiver_inside.lock().unwrap().recv().unwrap();
                println!("Thread-{}, received Job {} {:?}", id, job.id, job.command);
                match job.command {
                    Command::RUNNABLE => {
                        (job.runnable)();
                        println!("Thread-{}, finished RUNNABLE Job {}", id, job.id);
                    },
                    Command::INTERRUPT => {
                        break
                        println!("Thread-{}, finished INTERRUPT Job {}", id, job.id);
                    }
                }
                
            }
        });
        Worker { id, thread, receiver}
    }
}