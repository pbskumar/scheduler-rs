use crate::job::Job;
use crate::scheduler::policy::Policy;
use std::collections::VecDeque;

pub struct FifoPolicy {
    queue: VecDeque<Job>,
}

impl Policy for FifoPolicy {
    fn enqueue(&mut self, job: Job) {
        self.queue.push_back(job);
    }

    fn next_job(&mut self) -> Option<&Job> {
        self.queue.front()
    }
}
