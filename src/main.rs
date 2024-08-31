use clap::Parser;
use std::{
    thread::{self, JoinHandle},
    time,
};

#[derive(Parser)]
struct Args {
    /// Maximum threads.
    #[arg(short, long, default_value_t = 1)]
    count: u16,
    /// Thread sleep time(millisecond).
    #[arg(short, long, default_value_t = 10)]
    sleep: u16,
}
fn main() {
    let args = Args::parse();
    if args.count < 1 {
        panic!("arg count > 0")
    }
    let mut handlers = Vec::<JoinHandle<()>>::with_capacity(args.count as usize);
    let start = time::Instant::now();
    let pause = time::Duration::from_millis(args.sleep as u64);
    for _ in 0..args.count {
        let handle = thread::spawn(move || {
            thread::sleep(pause);
        });
        handlers.push(handle);
    }
    for handle in handlers {
        handle.join().unwrap();
    }
    let finish = time::Instant::now();
    println!("{:02?}", finish.duration_since(start));
}
