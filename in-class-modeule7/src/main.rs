/* task 1
fn main() {
    let operation = |a: i32, b: i32| {
        a * b
    };

    println!("Result: {}", operation(10, 5));
}
*/

/* task 2
    fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 1;
        println!("Counter total: {}", tracker);
    };

    update();
    update();
}

fn main() {
    track_changes();
}
*/

 //task 3
 
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Your implementation here
    
    let mut ans = Vec::new();

    for x in vec {
        ans.push(f(x));
    }
    ans
    
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
         x * 2
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2 {
            0
        }
        else {
            x
        }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
} 

/*
use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    // Add fields here
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        // Your implementation here
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        // Your implementation here
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.to_string()
            }
            None => {
                println!("Computing (this will take 2 seconds)...");
                thread::sleep(Duration::from_secs(2));
                let v = (self.computation)();
                self.value = Some(v.clone());
                v.to_string()
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
    */