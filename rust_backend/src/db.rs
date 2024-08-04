use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect(is_migrate: bool) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect("postgresql://ukxmhtalodjcvuizfvby:TiPwjkCCbmwqDB1wim3M4liub3115F@bpyai49hvux0pwemuajn-postgresql.services.clever-cloud.com:50013/bpyai49hvux0pwemuajn")
            .await
            .expect("Failed to connect to postgres.");

        println!("DB successfully connected ✅");

        if is_migrate {
            Self::migrate(&pool).await;
        }

        Database { pool }
    }

    pub async fn disconnect(&self) {
        self.pool.close().await;
        println!("DB connection disconnected successfully ✅");
    }

    pub async fn migrate(pool: &Pool<Postgres>) {
        match sqlx::migrate!("./migrations").run(pool).await {
            Ok(_) => println!("Migrations executed successfully."),
            Err(e) => eprintln!("Error executing migrations: {}", e),
        };
    }
}
