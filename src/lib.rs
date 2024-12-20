use std::net::IpAddr;
use eyre::Result;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod router;

pub struct App {
    address: IpAddr,
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        let address = IpAddr::from([127,0,0,1]);

        // Initialize tracing subscriber
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .init();

        Self {
            address, port
        }
    }

    pub async fn run(&self) -> Result<()> {
        let router = router::create_main_router();

        // use tokio
        let listener = tokio::net::TcpListener::bind((self.address, self.port)).await?;

        tracing::info!("Listening on {}", self.port);

        axum::serve(listener, router).await?;
        Ok(())
    }
}