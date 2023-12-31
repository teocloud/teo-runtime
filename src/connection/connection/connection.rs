use std::fmt::Debug;
use std::sync::Arc;
use async_trait::async_trait;
use crate::connection::transaction::Transaction;
use teo_result::Result;

#[async_trait]
pub trait Connection: Send + Sync + Debug {

    async fn transaction(&self) -> Result<Arc<dyn Transaction>>;

    async fn no_transaction(&self) -> Result<Arc<dyn Transaction>>;
}