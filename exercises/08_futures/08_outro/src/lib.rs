// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use without_channels::data;
use without_channels::store;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTicket {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicketResponse {
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetTicket {
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTicketResponse {
    pub ticket: Option<TicketDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketDetail {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchTicket {
    pub id: u64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Clone)]
pub struct Server {
    store: Arc<RwLock<store::TicketStore>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            store: Arc::new(RwLock::new(store::TicketStore::new())),
        }
    }

    pub async fn launch(self) -> std::net::SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        axum::serve(listener, app(self)).await.unwrap();
        addr
    }
}

pub fn app(server: Server) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ticket", post(create_ticket))
        .route("/get", post(get_ticket))
        .route("/patch", post(patch_ticket))
        .with_state(server)
}

// Handler: GET /health
async fn health_check(State(_): State<Server>) -> &'static str {
    "OK"
}

async fn create_ticket(
    State(state): State<Server>,
    Json(payload): Json<CreateTicket>,
) -> Json<CreateTicketResponse> {
    let id = state.store.write().await.add_ticket(data::TicketDraft {
        title: payload.title.try_into().unwrap(),
        description: payload.description.try_into().unwrap(),
    });
    Json(CreateTicketResponse { id: id.0 })
}

async fn get_ticket(
    State(state): State<Server>,
    Json(payload): Json<GetTicket>,
) -> Json<GetTicketResponse> {
    let ticket = state.store.read().await.get(store::TicketId(payload.id));
    match ticket {
        None => Json(GetTicketResponse { ticket: None }),
        Some(detail) => {
            let ticket = detail.read().unwrap();

            Json(GetTicketResponse {
                ticket: Some(TicketDetail {
                    id: ticket.id.0,
                    title: ticket.title.to_string(),
                    description: ticket.description.to_string(),
                    status: format!("{:?}", ticket.status),
                }),
            })
        }
    }
}

async fn patch_ticket(
    State(state): State<Server>,
    Json(payload): Json<PatchTicket>,
) -> Json<GetTicketResponse> {
    let ticket = state.store.write().await.get(store::TicketId(payload.id));
    match ticket {
        None => Json(GetTicketResponse { ticket: None }),
        Some(detail) => {
            let mut ticket = detail.write().unwrap();

            if let Some(title) = payload.title {
                ticket.title = title.try_into().unwrap();
            }

            if let Some(description) = payload.description {
                ticket.description = description.clone().try_into().unwrap();
            }

            if let Some(status) = payload.status {
                ticket.status = status.try_into().unwrap();
            }

            Json(GetTicketResponse {
                ticket: Some(TicketDetail {
                    id: ticket.id.0,
                    title: ticket.title.to_string(),
                    description: ticket.description.to_string(),
                    status: format!("{:?}", ticket.status),
                }),
            })
        }
    }
}
