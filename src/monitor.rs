use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::traits::Monitor;

pub struct MonitorRunner {
    monitors: Vec<Arc<Mutex<Box<dyn Monitor + Send + Sync>>>>,
}

impl MonitorRunner {
    //ToDo: Docs
    pub fn new() -> Self {
        MonitorRunner {
            monitors: Vec::new(),
        }
    }

    //ToDo: Docs
    pub fn add_component(&mut self, component: Arc<Mutex<Box<dyn Monitor + Send + Sync>>>) {
        self.monitors.push(component);
    }

    //ToDo: Docs
    pub async fn run(&self) -> Result<()> {
        for monitor in &self.monitors {
            // Clone monitor arc for spawn
            let monitor_clone = monitor.clone();

            // Start monitor in spawn
            tokio::spawn(async move {
                match monitor_clone.lock().await.monitor().await {
                    Ok(result) => info!("Monitoring was successful: {}", result),
                    Err(err) => warn!(
                        "Monitoring failed for {:#?} because: {}",
                        monitor_clone, err
                    ),
                }

                Ok::<(), anyhow::Error>(())
            });
        }

        Ok(())
    }
}
