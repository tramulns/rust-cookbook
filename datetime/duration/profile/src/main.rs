/// Измерение прошедшего времени между двумя моментами выполнения кода
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn expensive_function() {
    thread::sleep(Duration::from_secs(1));
}
