use axum::extract::FromRequestParts;
use parking_lot::{lock_api::ArcMutexGuard, RawMutex};

use super::error::Error;
use super::extension::{Extension, LazyTransaction};

pub struct Tx {
    tx: ArcMutexGuard<RawMutex, LazyTransaction>,
}


impl AsRef<sqlx::PgTransaction<'static>> for Tx {
    fn as_ref(&self) -> &sqlx::PgTransaction<'static> {
        self.tx.as_ref()
    }
}


impl std::ops::Deref for Tx {
    type Target = sqlx::PgTransaction<'static>;

    fn deref(&self) -> &Self::Target {
        self.tx.as_ref()
    }
}

impl std::ops::DerefMut for Tx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.tx.as_mut()
    }
}

impl<S>  FromRequestParts<S> for Tx
where 
    S: Sync
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ext: &Extension = parts.extensions.get().ok_or(Error::MissingExtension)?;

        let tx = ext.acquire().await?;
        Ok(Self {
            tx,
        })
    }
}
