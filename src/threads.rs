use std::thread;

pub fn start_basic_thread() {
    let mut x: u128 = 100u128;
    for i in 1..500_000 {
        x += i;
    }
    println!("Value in Main Thread:  {}", x);
}

pub fn spawn_thread() {
    let my_fn = || {
        let mut x: u128 = 100u128;
        for i in 1..500_000 {
            x += i;
        }
        println!("Value of x: {}", x);
    };
    let handle = thread::spawn(my_fn);
    let handle2 = thread::spawn(my_fn);

    start_basic_thread();
    println!("Starting worker thread :: {:?}...", handle.thread().id());
    handle.join();
    handle2.join();
}
