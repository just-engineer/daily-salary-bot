use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::any::Any;
use tokio::time::Duration;
use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use mockall::*;

#[async_trait]
#[automock]
pub trait Storage {
    async fn find_jobs(&self);
    async fn insert_new(&self, job_name: String, time: DateTime<Utc>);
}

pub struct Scheduler {
    name_job_map: HashMap<String, Box<dyn Fn()>>,
    storage: Box<dyn Storage>,
}


impl Scheduler {
    pub fn new(storage: Box<dyn Storage>) -> Scheduler {
        Scheduler {
            name_job_map: HashMap::new(),
            storage,
        }
    }

    pub fn register_job<T: Fn() + 'static>(&mut self, job: T, job_name: &str) {
        self.name_job_map.insert(job_name.to_owned(), Box::new(job));
    }

    pub fn schedule_job(&mut self, job_name: &str, time: DateTime<Utc>) {}

    fn execute_job(&mut self, job_name: &str) {
        let option = Some("hello");
        let x = option.unwrap();
        if let Some(func) = self.name_job_map.get(job_name) {
            func();
        }
    }

    async fn start() {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        while let _ = interval.tick().await {}
    }
}

#[test]
fn test() {
    let storage = MockStorage::new();
    let mut scheduler = Scheduler::new(Box::new(storage));
    let job_name = "hello_job";
    scheduler.register_job(|| println!("hello"), job_name);
    scheduler.execute_job(job_name);
}