use crate::time::Tick;

mod time;

fn main() {
    println!("Simulating time progress");

    let mut processing_time: Tick = 0;
    while processing_time < 10 {
        println!("Current time: {}", processing_time);
        processing_time += 1
    }
}
