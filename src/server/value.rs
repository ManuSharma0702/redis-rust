use std::{sync::mpsc, thread};

use crate::{command::CommandError, resp::ParseError};

#[derive(Debug)]
pub enum ServerError {
    Command(CommandError),
    Parse(ParseError),
    PoolCreationError,
}

pub struct Worker {
    pub id: usize,
    pub thread: thread::JoinHandle<()>,
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool{
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>
}

