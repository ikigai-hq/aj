//! Backend in AJ support both storage (backend) and queue (broker)

#![allow(clippy::borrowed_box)]

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum QueueDirection {
    Front,
    Back,
}

pub trait Backend {
    fn queue_push(&self, queue_name: &str, item: &str) -> Result<(), Error>;

    fn queue_move(
        &self,
        from_queue: &str,
        to_queue: &str,
        count: usize,
        from_position: QueueDirection,
        to_position: QueueDirection,
    ) -> Result<Vec<String>, Error>;

    fn queue_remove(&self, queue: &str, item: &str) -> Result<(), Error>;

    fn queue_get(&self, queue: &str, count: usize) -> Result<Vec<String>, Error>;

    fn queue_count(&self, queue: &str) -> Result<usize, Error>;

    fn storage_upsert(&self, hash: &str, key: &str, value: String) -> Result<(), Error>;

    fn storage_get(&self, hash: &str, key: &str) -> Result<Option<String>, Error>;
}

pub fn upsert_to_storage<T: Serialize>(
    backend: &dyn Backend,
    hash: &str,
    key: &str,
    value: T,
) -> Result<(), Error> {
    let value = serde_json::to_string(&value).unwrap_or("".into());
    backend.storage_upsert(hash, key, value)
}

pub fn get_from_storage<T: DeserializeOwned>(
    backend: &dyn Backend,
    hash: &str,
    key: &str,
) -> Result<Option<T>, Error> {
    let value = backend.storage_get(hash, key).ok().flatten();
    let item = match value {
        Some(value) => serde_json::from_str(&value).ok(),
        _ => None,
    };
    Ok(item)
}
