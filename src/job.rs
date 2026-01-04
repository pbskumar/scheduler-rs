use crate::job::JobEvent::{Finished, Ran};
use crate::job::TaskType::CPU;
use crate::time::Tick;

#[derive(Debug)]
pub enum TaskType {
    CPU(Tick),
    // IO(Tick) - Implementing only CPU tasks for now
}

pub enum JobEvent {
    Ran(Tick),
    Blocked,
    Finished,
}

#[derive(Debug)]
pub struct Job {
    id: usize,
    pub name: String,
    tasks: Vec<TaskType>,
}

impl Job {
    pub fn new(id: usize, name: String, tasks: Vec<TaskType>) -> Job {
        Job {id, name, tasks}
    }

    pub fn step(&mut self, tick_quota: Tick) -> JobEvent {
        if self.tasks.is_empty() {
            return Finished;
        }

        let mut remaining_quota: Tick = tick_quota;
        while remaining_quota > 0 {
            if self.tasks.is_empty() {
                break;
            }
            println!("Job: {}, Tasks: {:?}", self.name, self.tasks);
            let next_task = self.tasks.first().unwrap();
            let cycles ;
            match next_task {
                CPU(tick) => {
                    if *tick > remaining_quota {
                        self.tasks[0] = CPU(*tick - remaining_quota);
                        cycles = remaining_quota;
                    } else {
                        cycles = *tick;
                        self.tasks.remove(0);
                    }

                    println!("Processed {} for {} ticks", self.name, cycles);
                },
            }
            remaining_quota -= cycles;
        }
        Ran(tick_quota - remaining_quota)
    }
}
