use chrono::{Utc, DateTime};
use std::collections::HashMap;
use std::any::Any;
use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use mockall::*;
use std::time::Duration;
use tokio::task::JoinHandle;

#[async_trait]
#[automock]
pub trait Storage {
    async fn find_jobs(&self, from: DateTime<Utc>, to: DateTime<Utc>) -> Vec<JobEntity>;
    async fn insert_new(&self, job_name: String, time: DateTime<Utc>);
}

pub struct JobEntity {
    time: DateTime<Utc>,
    job_name: String
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

    pub fn schedule_job(&mut self, job_name: &str, time: DateTime<Utc>) {
        self.storage.insert_new(job_name.to_owned(), time);
    }

    fn execute_job(&mut self, job_name: &str) {
        let option = Some("hello");
        let x = option.unwrap();
        if let Some(func) = self.name_job_map.get(job_name) {
            func();
        }
    }

    async fn start(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        while let _ = interval.tick().await {
            let now = Utc::now();
            let jobs = self.storage.find_jobs(now, now + chrono::Duration::seconds(15)).await;

            let x1 = jobs.iter()
                .filter_map(|entity| {
                    match self.name_job_map.get(&entity.job_name) {
                        Some(func) => {
                            let join_handle = tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                func();
                            });
                            Some(join_handle)
                        }
                        None => {println!("job {} not found", entity.job_name); None},
                    }
                })
                .collect::<Vec<_>>();
            tokio::join![x1]
        }
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

fn test1() {
    let storage = MockStorage::new();
    let mut scheduler = Scheduler::new(Box::new(storage));
    let job_name = "hello_job";
    scheduler.register_job(|| println!("hello"), job_name);
    scheduler.schedule_job(job_name, Utc::now());

}