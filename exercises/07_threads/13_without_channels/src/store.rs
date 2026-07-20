use std::collections::BTreeMap;
use std::sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::data::{Status, Ticket, TicketDraft};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(pub u64);

/// The state behind the store's single RwLock.
pub struct TicketData {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}

impl TicketData {
    pub fn new() -> RwLock<Self> {
        RwLock::new(Self {
            tickets: BTreeMap::new(),
            counter: 0,
        })
    }

    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: draft.title,
            description: draft.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }
}

pub struct TicketStore {
    data: RwLock<TicketData>,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            data: TicketData::new(),
        }
    }
    pub fn read(
        &self,
    ) -> Result<RwLockReadGuard<'_, TicketData>, PoisonError<RwLockReadGuard<'_, TicketData>>> {
        self.data.read()
    }

    pub fn write(
        &self,
    ) -> Result<RwLockWriteGuard<'_, TicketData>, PoisonError<RwLockWriteGuard<'_, TicketData>>>
    {
        self.data.write()
    }
}
