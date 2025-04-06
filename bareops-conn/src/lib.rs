use std::sync::Arc;

use tokio::sync::Mutex;

pub(crate) mod rpc;
pub(crate) mod ssh;

/// Models the connection to a single remote bareops instance
pub struct Connection {}

impl Connection {
    fn send(&mut self) {}
}

/// Handles connections to (possibly multiple) remote bareops instances
pub struct ConnectionManager {
    conns: Vec<Arc<Mutex<Connection>>>,
}
