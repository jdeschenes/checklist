use std::sync::Arc;

use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex};
use sqlx::PgTransaction;

use super::error::Error;
use super::state::State;

pub struct Extension {
    slot: Arc<Mutex<LazyTransaction>>,
}

impl Extension {
    pub fn new(state: State) -> Self {
        Self {
            slot: Arc::new(Mutex::new(LazyTransaction::new(state))),
        }
    }

    pub async fn acquire(&self) -> Result<ArcMutexGuard<RawMutex, LazyTransaction>, Error> {
        let mut tx = self.slot.try_lock_arc().ok_or(Error::OverlappingExtractors)?;
        tx.acquire().await?;
        Ok(tx)
    }

    pub async fn resolve(&self) -> Result<(), Error> {
        if let Some(mut tx) = self.slot.try_lock_arc() {
            tx.resolve().await?;
        }
        Ok(())
    }
}

impl Clone for Extension {
    fn clone(&self) -> Self {
        Self {
            slot: self.slot.clone(),
        }
    }
}


pub struct LazyTransaction(LazyTransactionState);

enum LazyTransactionState {
    Unacquired {
        state: State,
    },
    Acquired {
        tx: PgTransaction<'static>,
    },
    Resolved,
}

impl LazyTransaction {
    pub fn new(state: State) -> Self {
        Self(LazyTransactionState::Unacquired { state })
    }

    pub(crate) fn as_ref(&self) -> &PgTransaction<'static> {
        match &self.0 {
            LazyTransactionState::Unacquired { .. } | LazyTransactionState::Resolved => panic!("BUG: transaction is not acquired"),
            LazyTransactionState::Acquired { tx } => tx,
        }
    }

    pub(crate) fn as_mut(&mut self) -> &mut PgTransaction<'static> {
        match &mut self.0 {
            LazyTransactionState::Unacquired { .. } | LazyTransactionState::Resolved => panic!("BUG: transaction is not acquired"),
            LazyTransactionState::Acquired { tx } => tx,
        }
    }

    async fn acquire(&mut self) -> Result<(), Error> {
        match &self.0 {
            LazyTransactionState::Unacquired { state } => {
                let tx = state.transaction().await?;
                self.0 = LazyTransactionState::Acquired { tx };
                Ok(())
            },
            LazyTransactionState::Acquired { .. } => Ok(()),
            LazyTransactionState::Resolved => Err(Error::OverlappingExtractors),
        }
    }

    pub async fn resolve(&mut self) -> Result<(), sqlx::Error> {
        match std::mem::replace(&mut self.0, LazyTransactionState::Resolved) {
            LazyTransactionState::Unacquired { .. } | LazyTransactionState::Resolved => Ok(()),
            LazyTransactionState::Acquired { tx } => {
                tx.commit().await?;
                Ok(())
            },
        }
    }

    pub async fn commit(&mut self) -> Result<(), sqlx::Error> {
        match std::mem::replace(&mut self.0, LazyTransactionState::Resolved) {
            LazyTransactionState::Unacquired { .. } => {
            panic!("BUG: tries to commit an unaquired transaction")
        },
        LazyTransactionState::Acquired { tx } => tx.commit().await,
        LazyTransactionState::Resolved => panic!("BUG: tries to commit a resolved transaction"),
    }
    }
}
