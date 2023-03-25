static mut GLOBAL_COUNTER: usize = 0;

fn counter_inc() {
    unsafe {
        GLOBAL_COUNTER += 1;
    }
}

fn counter_get() -> usize {
    unsafe { GLOBAL_COUNTER }
}

fn main() {
    let mut handles = vec![];
    for _ in 0..5 {
        handles.push(std::thread::spawn(|| {
            for _ in 0..1000 {
                counter_inc();
            }
            counter_get()
        }));
    }

    let mut counter = 0;
    handles
        .into_iter()
        .for_each(|h| counter += h.join().unwrap());

    println!("counter = {}", counter);
}