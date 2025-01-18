use rand;
use std::thread;
use clap::Parser;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    /// Number of workers to start
    #[arg(short, long)]
    count: u8,

}

/// Puts working load on given number of CPU threads
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

    println!("{n} workers started");

    for handle in handles {
        handle.join().unwrap();
    }

}

fn main() {
    let args = Args::parse();

    let c = args.count;

    stress_this(c);

}
