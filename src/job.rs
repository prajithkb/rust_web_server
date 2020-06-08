use crate::command::Command;
use crate::thread_pool::Runnable;
use std::fmt::Debug;

pub struct Job {
    pub runnable: Runnable,
    pub id: String,
    pub command: Command,
}

impl Debug for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Job")
            .field("runnable", &"Runnable")
            .field("id", &self.id)
            .field("command", &self.command)
            .finish()
    }
}

impl Job {
    pub fn new(runnable: Runnable, id: String, command: Command) -> Job {
        Job {
            runnable,
            id,
            command,
        }
    }
}


