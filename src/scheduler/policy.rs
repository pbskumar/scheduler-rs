use crate::job::{Job, JobEvent};
use crate::time::Tick;

mod fifo_policy;

pub trait Policy {

    fn enqueue(&mut self, job: Job);

    fn next_job(&mut self) -> Option<&Job>;
}
