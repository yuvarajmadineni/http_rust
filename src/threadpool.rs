use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

// workers will process the jobs
// notify workers when jobs arrive , they will take message from the queue and process
pub struct Threadpool {
    pub workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    pub id: u8,
    pub thread: std::thread::JoinHandle<()>,
}

impl Threadpool {
    pub fn new(size: u8) -> Self {
        let (tx, rx) = channel();
        let mut workers = Vec::new();
        let receiver = Arc::new(Mutex::new(rx));
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        Threadpool {
            workers,
            sender: Some(tx),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Worker {
    pub fn new(id: u8, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => job(),
                Err(e) => {
                    eprintln!("Error while receiving message from channel {:?}", e);
                }
            }
        });
        Worker { id, thread }
    }
}
