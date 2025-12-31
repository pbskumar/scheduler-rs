# Rust based Scheduler Simulator

This is a Scheduler simulation project to implement various scheduling policies. 
The project will have two components: Scheduler, Synthetic Job Submitter.

I plan to implement the following scheduling policies and gather metrics on the effectiveness of these policies.

## Inspiration
**Operating Systems: Three Easy Pieces** discusses these policies. I'm learning Rust and deep diving into systems. 
This project is a way to apply my new learnings! 

## Assumptions
1. Single-core processing

## Scheduling policies

| Policy                           | Description                                                                                                         | Preemptive | Implemented (Y/N) |
|----------------------------------|---------------------------------------------------------------------------------------------------------------------|:-----------|-------------------|
| Cooperative Scheduling           | Scheduler assumes job are fair and yield cooperatively.                                                             | N          | N                 |
| FIFO                             | Jobs are scheduled and processed int the order of submission.                                                       | N          | N                 |
| Round Robin                      | Each job gets a fixed quantum of execution time and preempted after it.                                             | Y          | N                 |
| Shortest Job First (SJF)         | Always schedules the first process with smallest expected execution time. <br/>Assumes the execution time is known. | N          | N                 |
| Shortest Time to Completion      | Preemptive version of SJF. Always runs the next process that is expected to complete sooner.                        | Y          | N                 |
| Multilevel Feedback Queue (MLFQ) | Dynamically adjust job priority and scheduling based on observed behavior.                                          | Y          | N                 |
| Lottery Scheduling               | Assigns random tickets to jobs and processed based on the ticket picked.                                            | TBD        | N                 |
| Completely Fair Scheduler (CFS)  | Linux version of scheduling policy aimed at fair CPU distribution using virtual runtiem and Red Black Trees.        | Y          | N                 |

## Performance Metrics

| Metric           | Description                                                                                  | Implemented (Y/N) |
|------------------|----------------------------------------------------------------------------------------------|-------------------|
| Response time    | Time from submission of a job to its first execution.                                        | N                 |
| Turnaround time  | Total time from job submission to completion.                                                | N                 |
| Jain's index     | Measures fairness of resurce allocation among processes. <br/>Range: [Unfair (0) - Fair (1)] | N                 |
| Context Switches | Number of times the scheduler preempted jobs.                                                | N                 |
| Starvation time  | Time a job waits without being scheduled due to low priority.                                | N                 |

## Design

In Progress...
