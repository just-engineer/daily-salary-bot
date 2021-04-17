use tokio_postgres::{Client, NoTls};
use chrono::{Utc, DateTime};
use async_trait::async_trait;
use crate::scheduler::Storage;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

struct PostgresStorage {
    client: Client,
}

impl PostgresStorage {
    pub async fn new() -> Self {
        let url = env!("POSTGRES_URL");
        let (mut client, connection) =
            tokio_postgres::connect(url, NoTls).await.unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        embedded::migrations::runner()
            .run_async(&mut client).await.expect("migrations fail =(");

        PostgresStorage { client }
    }

}
#[async_trait]
impl Storage for PostgresStorage {
    async fn find_jobs(&self, from: DateTime<Utc>, to: DateTime<Utc>) {
        todo!()
    }

    async fn insert_new(&self, job_name: String, time: DateTime<Utc>) {
        todo!()
    }
}


#[tokio::test]
async fn test() {
    let storage = PostgresStorage::new().await;
}
