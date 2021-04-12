use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::any::Any;
use tokio::time::Duration;
use std::fmt::{Display, Formatter};
use std::panic::resume_unwind;

struct Scheduler {
    name_job_map: HashMap<String, Box<dyn Fn()>>,
}


impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            name_job_map: HashMap::new(),
        }
    }

    pub fn register_job<T: Fn() + 'static>(&mut self, job: T, job_name: &str) {
        self.name_job_map.insert(job_name.to_owned(), Box::new(job));
    }

    fn execute_job(&mut self, job_name: &str) {
        let option = Some("hello");
        let x = option.unwrap();
        if let Some(func) = self.name_job_map.get(job_name) {
            func();
        }
    }

    async fn start() {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        while let _ = interval.tick().await {

        }
    }
}

#[test]
fn test() {
    let mut scheduler = Scheduler::new();
    let job_name = "hello_job";
    scheduler.register_job(|| println!("hello"), job_name);
    scheduler.execute_job(job_name);
}