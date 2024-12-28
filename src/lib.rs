use std::net::IpAddr;
use eyre::Result;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::db::connect::{connect_to_db, DB};
use crate::state::AppState;

mod router;
mod db;
mod state;

pub struct App {
    address: IpAddr,
    port: u16,
    db: DB,
}

impl App {
    pub async fn new(port: u16, database_uri: &str) -> Result<Self> {
        let address = IpAddr::from([127,0,0,1]);
        
        // create connection to database
        let db = connect_to_db(database_uri).await?; 

        // Initialize tracing subscriber
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        Ok(
            Self { 
                address, port, db 
            }
        )
    }

    pub async fn run(&self) -> Result<()> {
        let state: AppState = AppState {
            db: self.db.clone(),
        };

        let router = router::create_main_router(state);

        // use tokio
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await?;

        tracing::info!("Listening on {}", self.port);

        axum::serve(listener, router).await?;
        Ok(())
    }
}