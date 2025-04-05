// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Create an Arc that wraps a Mutex protecting the JobStatus
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        // Spawn a new thread
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250)); // Sleep without holding the lock
            // Lock the Mutex and increment the jobs_completed
            let mut job_status = status_shared.lock().unwrap(); // Lock the Mutex
            job_status.jobs_completed += 1; // Increment the counter
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for all threads to finish
    }

    // After all threads have finished, safely access the jobs_completed
    let final_jobs_completed = status.lock().unwrap().jobs_completed;
    println!("jobs completed {}", final_jobs_completed);
}