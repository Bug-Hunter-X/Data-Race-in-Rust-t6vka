use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    let y = x.clone();
    let z = x.clone();

    let y_thread = std::thread::spawn(move || {
        *y.lock().unwrap() = 10;
    });
    let z_thread = std::thread::spawn(move || {
        *z.lock().unwrap() = 15;
    });

    y_thread.join().unwrap();
    z_thread.join().unwrap();
    println!("Value: {}", *x.lock().unwrap());
} 