use crate::store::TicketId;
use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = StatusError;
    fn try_from(value: String) -> Result<Self, StatusError> {
        match value.to_uppercase().as_str() {
            "TODO" => Ok(Status::ToDo),
            "INPROGRESS" => Ok(Status::InProgress),
            "DONE" => Ok(Status::Done),
            _ => Err(StatusError::InvalidStatusError),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StatusError {
    #[error("Invalid status")]
    InvalidStatusError,
}
