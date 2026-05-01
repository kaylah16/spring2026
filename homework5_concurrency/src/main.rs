//assignment3
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel(); //creates channel
        let receiver = Arc::new(Mutex::new(receiver)); //allows receiver to be shared by multiple workers
        
        // TODO: Create and store workers
        let mut workers = vec![]; //new vector to store workers

        for i in 1..=size {
            let id = i; // worker's id
            let cloned_receiver = receiver.clone(); //allows each worker to own a receiver
            let new_worker = Worker::new(id, cloned_receiver); //create new worker
            workers.push(new_worker); //push worker to vector
        }
        
        // TODO: Return the ThreadPool
        ThreadPool{workers, sender} //returns completeted pool
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        let job = Box::new(f); //creates a box and stores f (f is closure?)
        let _ = self.sender.send(Message::NewJob(job)); //create message and sent to worker in channel
            //side note, was getting a note for not have declared as variable(?)
        
    }
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {  //look up how to do this
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
        for worker in &mut self.workers { //iterate through each worker
            self.sender.send(Message::Terminate).unwrap(); //send message to workers 
        // TODO: Wait for all workers to finish
            if let Some(thread) = worker.thread.take() { //checks for handle & get joinhandle out from option
                thread.join().unwrap(); //wait for worker to be done
            }
       }
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || { //create a thread
            loop { //loop to get messages and handle them
                let message = receiver.lock().unwrap(); //get message from locked receiver
                match message.recv().unwrap() {
                    Message::Terminate => break, //worker is done
                    Message::NewJob(job) => { //worker has a job
                        println!("worker {} is doing a job.", id); 
                        job(); //does job that workers needs to do
                    }
                }
            }
        });
        
        // TODO: Return the Worker
        Worker{id, thread: Some(thread)} //stores joinhandle 
        
    }
}

fn main() {
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup
}


/*
//assignment 4

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel(); //create channel

    let rx = Arc::new(Mutex::new(rx)); //receiver shared by consumer
    
    
    // TODO: Create 2 producer threads
    let mut prod_handle = vec![]; //create vector to insert producers

     for i in 0..2 { //create 2 producers
        let tx_clone = tx.clone(); //clone transmitter
        let handle = thread::spawn(move || { //create new thread
            producer(i, tx_clone, ITEM_COUNT);
        });
        prod_handle.push(handle); //push to handle
    }
    
    
    // TODO: Create 3 consumer threads
    let con = 3;
    let mut con_handle = vec![]; //create vector to insert consumer
    for i in 1..=con {
        let rx_clone = rx.clone(); //clone receiver to give to consumer

        let handle = thread::spawn(move || { //create new thread
            consumer(i, rx_clone);
        });
        con_handle.push(handle); //push to handle
    }
    
    
    // TODO: Wait for all threads to finish
    for handle in prod_handle { //wait for producer to finish first
        handle.join().unwrap();
    }

    for _ in 0..con { //send termination signal to consumer
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in con_handle { //wait for consumer to finish
        handle.join().unwrap();
    }


    println!("Everything has been terminated");
    
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal

    let mut rng = rand::thread_rng();

    for _ in 0..=item_count{ //iterate through every item
        let r = rng.gen_range(0..100); //generate random number

        tx.send(r).unwrap(); //value sent to receving consumer
        println!("producer {} send {}", id, r);

    }

    println!("producer {} completed its task", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal

    loop {
        let received_value = rx.lock().unwrap(); //lock receiver to get value

        if received_value.recv().unwrap() == TERMINATION_SIGNAL { //check for termination value
                println!("Consumer {} is exiting", id); //break if value equal to signal
                break;
            }
            else {
                println!("Consumer {} is continuing process", id); //else continue process
            }
    }
}
    */