mod health;

use crate::health::health;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::watch};
use zksync_os_observability::ComponentHealth;
use zksync_os_pipeline_health::ComponentId;
use zksync_os_types::TransactionAcceptanceState;

#[derive(Clone)]
pub(crate) struct AppState {
    pub stop_receiver: watch::Receiver<bool>,
    pub acceptance_state: watch::Receiver<TransactionAcceptanceState>,
    pub component_health: Arc<Vec<(ComponentId, watch::Receiver<ComponentHealth>)>>,
}

pub async fn run_status_server(
    bind_address: String,
    stop_receiver: watch::Receiver<bool>,
    acceptance_state: watch::Receiver<TransactionAcceptanceState>,
    component_health: Arc<Vec<(ComponentId, watch::Receiver<ComponentHealth>)>>,
) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/status/health", get(health))
        .with_state(AppState {
            stop_receiver,
            acceptance_state,
            component_health,
        });

    let addr: SocketAddr = bind_address.parse()?;
    let listener = TcpListener::bind(addr).await?;

    let addr = listener.local_addr()?;
    tracing::info!("running a status server" = %addr);

    axum::serve(listener, app).await?;

    Ok(())
}
