extern crate chrono;
use std::time::{Duration, Instant};

pub fn test_time_rust_module() {
    let dur1 = Duration::from_secs(15);
    println!("Time Duration : {}", dur1.as_secs());

    let dur2 = Duration::from_millis(1000);
    println!("Time Duration : {}", dur2.as_secs());

    let dur3 = dur1.checked_sub(dur2);
    match dur3 {
        Some(dur) => println!("Time Duration after sub : {}", dur.as_secs()),
        None => println!("Time Duration : None"),
    }

    // Instant
    let current_time = Instant::now();
    println!("Current Time : {:?}", current_time);
    std::thread::sleep(Duration::from_secs(2));
    println!("Current Time : {:?}", current_time.elapsed());
}

pub fn test_chrono() {
    let local_time = chrono::Local::now();
    println!(
        "My Local Time : {}",
        local_time.format("%b %d %Y - %H:%M:%S AM")
    );

    let utc_now = chrono::Utc::now();
    println!("UTC Time : {}", utc_now.format("%b %d %Y - %H:%M:%S AM"));
}
