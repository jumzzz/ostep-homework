use std::sync::{Arc,Mutex};
use std::thread;

fn mutex_as_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;    
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result = {:?}", *counter.lock().unwrap());
}



fn main() {
    mutex_as_counter();
}

