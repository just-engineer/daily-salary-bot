use chrono::{Utc, FixedOffset, TimeZone, NaiveTime, NaiveDateTime, DateTime, NaiveDate, Local};
use std::time::{Instant, Duration, SystemTime};
use std::ops::Add;
use std::convert::{TryFrom, TryInto};
use telegram_bot::InlineQueryResult::InlineQueryResultCachedSticker;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{UnboundedSender, UnboundedReceiver};
use futures::{StreamExt, FutureExt};
use tokio::join;

pub struct SchedulerBuilder {
    jobs: Vec<Job>
}

impl SchedulerBuilder {
    pub fn new() -> SchedulerBuilder {
        SchedulerBuilder { jobs: Vec::new() }
    }

    pub fn get_tick(&mut self, time: NaiveTime, offset: FixedOffset) -> UnboundedReceiver<()> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        self.jobs.push(Job { time, offset, tx });
        rx
    }

    pub fn build(self) -> Scheduler {
        Scheduler {
            jobs: Arc::new(self.jobs)
        }
    }

}

pub struct Scheduler {
    jobs: Arc<Vec<Job>>
}

impl Scheduler {
    pub async fn start(self) {
        let mut interval = tokio::time::interval_at(tokio::time::Instant::now(), tokio::time::Duration::from_secs(1));
        loop {
            let jobs_clone = Arc::clone(&self.jobs);
            interval.tick().await;
            println!("text");
            for x in &*jobs_clone {
                x.tx.send(());
            }
        }
    }
}


struct Job {
    time: NaiveTime,
    offset: FixedOffset,
    tx: UnboundedSender<()>,
}

fn parse(settings: String) {
    let naive_time = NaiveTime::parse_from_str(&settings, "%H:%M").expect("something wrong");


    let date = Utc::today() + chrono::Duration::days(1);
    let tomorrow_time = date.and_time(naive_time + chrono::FixedOffset::east(3 * 3600))
        .expect("something wrong");
    let i = tomorrow_time.timestamp_millis();
}

pub trait ScheduledJob {
    fn run();
}

#[tokio::test]
async fn test_scheduler() {
    let mut scheduler = SchedulerBuilder::new();
    let mut receiver = scheduler.get_tick(NaiveTime::from_hms(0, 0, 0), FixedOffset::east(0));
    let scheduler = scheduler.build().start();

    join![scheduler, loop_events(receiver)];

}

async fn loop_events(mut receiver: UnboundedReceiver<()>) {
    loop {
        let next = receiver.next().await;
        let time = Utc::now();
        eprintln!("time = {:?}", time);
    }
}