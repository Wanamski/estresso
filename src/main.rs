use rand;
use std::thread;

/// performs stress test on given number of threads
fn stress_this(n: u8) {
    let mut handles = vec![];

    for _ in 0..=n {
        let handle = thread::spawn(|| {
            // putting some load on the thread
            #[allow(unused)]
            loop {
                f64::sqrt(rand::random());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

}

fn main() {

    stress_this(16);

}
