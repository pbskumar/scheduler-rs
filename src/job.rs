use crate::job::JobEvent::{Finished, Ran};
use crate::job::TaskType::{CPU, IO};
use crate::time::Tick;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TaskType {
    CPU(Tick),
    IO(Tick)
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
        println!("Processing request to create new Job {} with {:?} tasks", name, tasks);
        let collapsed_jobs = Job::collapse_tasks(tasks);
        println!("Optimized and creating Job {} with {:?} tasks", id, collapsed_jobs);

        Job {id, name, tasks: collapsed_jobs }
    }

    fn collapse_tasks(tasks: Vec<TaskType>) -> Vec<TaskType> {
        tasks.into_iter().fold(Vec::new(), |mut acc, item| {
            match (acc.last_mut(), item) {
                (Some(TaskType::CPU(sum)), TaskType::CPU(x)) => {
                    *sum += x;
                }
                (Some(TaskType::IO(sum)), TaskType::IO(x)) => {
                    *sum += x;
                }
                (_, item) => {
                    acc.push(item)
                }
            }
            acc
        })
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
                    // Leaving duplicate code for now as this might move to Policy
                    // Keeping things fluid for now...
                    if *tick > remaining_quota {
                        self.tasks[0] = CPU(*tick - remaining_quota);
                        cycles = remaining_quota;
                    } else {
                        cycles = *tick;
                        self.tasks.remove(0);
                    }

                    println!("Processed CPU cycle of {} for {} ticks", self.name, cycles);
                },
                IO(tick) => {
                    if *tick > remaining_quota {
                        self.tasks[0] = IO(*tick - remaining_quota);
                        cycles = remaining_quota;
                    } else {
                        cycles = *tick;
                        self.tasks.remove(0);
                    }

                    println!("Blocked IO cycle of {} for {} ticks", self.name, cycles);
                }
            }
            remaining_quota -= cycles;
        }
        Ran(tick_quota - remaining_quota)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_test_job() {
        let job: Job = Job::new(
            1,
            String::from("job"),
            vec![CPU(1), CPU(2), IO(3), CPU(8), IO(1), CPU(2), CPU(7), IO(12)]
        );

        let expected_job_list = vec![CPU(3), IO(3), CPU(8), IO(1), CPU(9), IO(12)];
        assert_eq!(expected_job_list, job.tasks)
    }

}
