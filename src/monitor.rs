use anyhow::Result;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::time::Duration;
use tracing::{info, warn};

use crate::traits::Monitor;

pub struct MetricsExporter {
    monitors: Vec<Box<dyn Monitor + Send + Sync>>,
}

impl MetricsExporter {
    pub fn new() -> Self {
        MetricsExporter {
            monitors: Vec::new(),
        }
    }

    pub fn add_monitor(&mut self, component: Box<dyn Monitor + Send + Sync>) {
        self.monitors.push(component);
    }

    pub async fn run(&self) -> Result<()> {
        let builder = PrometheusBuilder::new();
        Ok(builder.install()?)
    }
}
