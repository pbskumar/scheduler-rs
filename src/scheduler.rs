use crate::job::{Job, JobEvent};
use crate::time::Tick;

pub mod policy;
pub mod handler;

pub struct Scheduler {
    ticker: Tick,
    jobs: Vec<Job>,
    time_slice: Tick
}

impl Scheduler {
    pub fn new(jobs: Vec<Job>) -> Self {
        Scheduler { ticker: 0, jobs, time_slice: 5 }
    }

    pub fn run(&mut self) {
        // naive implementation of run
        let mut current_tick = self.ticker;

        loop {
            if self.jobs.is_empty() { break; }

            println!("Current Tick: {}, Jobs in Queue: {:?}\n", current_tick, self.jobs);

            let next_job = self.jobs.first_mut().unwrap();

            let event: JobEvent = next_job.step(self.time_slice);

            match event {
                JobEvent::Ran(t) => {
                    println!("Job {} ran for {} ticks.", next_job.name, t);
                    current_tick += t;
                }
                JobEvent::Blocked => {
                    println!("Job {} blocked.", next_job.name)
                }
                JobEvent::Finished => {
                    println!("Job {} finished. Removing from task list", next_job.name);
                    self.jobs.remove(0);
                }
            }
            println!("Current tick: {}", current_tick);
            println!("\n------------------------\n")

        }

        println!("Scheduler ran for {} ticks.", current_tick);
    }
}
