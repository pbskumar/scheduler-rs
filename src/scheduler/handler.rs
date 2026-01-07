use crate::job::{Job, JobEvent};

pub trait Handler {
    fn run(job: &Job) -> JobEvent;
}
