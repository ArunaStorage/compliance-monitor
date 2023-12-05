/*
use crate::traits::Monitor;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct PermissionMonitor {}

#[async_trait::async_trait]
impl Monitor for PermissionMonitor {
    async fn monitor(&mut self) -> Result<bool> {
        // Start tokio spawn
        Ok(true)
    }
}

impl PermissionMonitor {
    pub fn new(host: String) -> Result<Self> {
        Ok(PermissionMonitor {})
    }
}
 */
