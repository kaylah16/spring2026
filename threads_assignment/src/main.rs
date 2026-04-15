//assignment 1
use std::thread;
//use std::time::Duration;
use std::sync::Mutex;
use std::sync::Arc;
fn main() {
    /*let mut handles = vec![];

    for i in 1..=3 { //copy i
        let handle = thread::spawn(move ||{
            println!("Thread {}", i);
        });
        handles.push(handle);

    }
     for handle in handles {
            handle.join().unwrap();
        }

    println!("All threads completed");
    */
    
    //asignment 2
    let total = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
         let cnt = total.clone();
        let handle = thread::spawn(move || {
           for _ in 0..10 {
            *cnt.lock().unwrap() += 1;
           }

        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *total.lock().unwrap());

}
