use without_channels::data::TicketDraft;
use without_channels::store::{TicketId, TicketStore};

use std::sync::Arc;
use tokio::net::TcpListener;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

// Define your JSON payload structs
#[derive(Serialize, Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTicketResponse {
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GetTicket {
    pub id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GetTicketResponse {
    pub ticket: Option<TicketDetails>,
}

#[derive(Serialize, Deserialize)]
pub struct TicketDetails {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct PatchTicket {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

pub struct Server {
    store: Arc<TicketStore>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            store: Arc::new(TicketStore::new()),
        }
    }

    pub async fn server(&self) -> SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        println!("Listening on http://{}", addr);

        // Spawn the serve loop so this method can hand back the bound addr.
        // The listener is already bound, so callers can connect right away.
        let app = self.app();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        addr
    }

    pub fn app(&self) -> Router {
        // Build routes and attach handlers
        Router::new()
            .route("/health", get(health_check))
            .route("/create", post(create_ticket))
            .route("/get", post(get_ticket))
            .route("/patch", post(patch_ticket))
            .with_state(self.store.clone())
    }
}

async fn health_check() -> &'static str {
    "OK"
}

async fn create_ticket(
    State(store): State<Arc<TicketStore>>,
    Json(payload): Json<CreateTicket>,
) -> Json<CreateTicketResponse> {
    let id = store.write().unwrap().add_ticket(TicketDraft {
        title: payload.title.try_into().unwrap(),
        description: payload.description.try_into().unwrap(),
    });

    Json(CreateTicketResponse { id: id.0 })
}

async fn get_ticket(
    State(store): State<Arc<TicketStore>>,
    Json(payload): Json<GetTicket>,
) -> Json<GetTicketResponse> {
    let id = TicketId(payload.id);
    match store.read().unwrap().get(id) {
        Some(ticket) => {
            let ticket = ticket.read().unwrap();
            Json(GetTicketResponse {
                ticket: Some(TicketDetails {
                    id: ticket.id.0,
                    title: ticket.title.clone().try_into().unwrap(),
                    description: ticket.description.clone().try_into().unwrap(),
                    status: ticket.status.to_string(),
                }),
            })
        }
        None => Json(GetTicketResponse { ticket: None }),
    }
}

async fn patch_ticket(
    State(store): State<Arc<TicketStore>>,
    Json(payload): Json<PatchTicket>,
) -> Json<GetTicketResponse> {
    let id = TicketId(payload.id);
    match store.read().unwrap().get(id) {
        Some(ticket) => {
            let mut ticket = ticket.write().unwrap();
            if let Some(title) = payload.title {
                ticket.title = title.try_into().unwrap();
            }
            if let Some(description) = payload.description {
                ticket.description = description.try_into().unwrap();
            }
            if let Some(status) = payload.status {
                ticket.status = status.try_into().unwrap();
            }
            Json(GetTicketResponse {
                ticket: Some(TicketDetails {
                    id: ticket.id.0,
                    title: ticket.title.clone().try_into().unwrap(),
                    description: ticket.description.clone().try_into().unwrap(),
                    status: ticket.status.to_string(),
                }),
            })
        }
        None => Json(GetTicketResponse { ticket: None }),
    }
}
