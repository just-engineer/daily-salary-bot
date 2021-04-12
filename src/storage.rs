use tokio_postgres::{Client, NoTls};

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

trait Storage {
    async fn find_jobs();
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

impl Storage for PostgresStorage {
    async fn find_jobs() {
        todo!()
    }
}


#[tokio::test]
async fn test() {
    let storage = PostgresStorage::new().await;
}
