extern crate core;

use crate::job::Job;
use crate::job::TaskType::CPU;
use crate::scheduler::Scheduler;

mod time;
mod scheduler;
mod job;

fn main() {
    println!("Simulating Job scheduler");

    let job1: Job = Job::new(
        1,
        String::from("job1"),
        vec![CPU(1), CPU(2), CPU(8), CPU(2), CPU(7)]
    );
    let job2: Job = Job::new(
        2,
        String::from("job2"),
        vec![CPU(1), CPU(17), CPU(4), CPU(21), CPU(3)]
    );

    let mut scheduler = Scheduler::new(vec![job1, job2]);
    scheduler.run();
}
