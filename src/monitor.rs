use anyhow::Result;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::time::Duration;
use tracing::info;
use metrics::{gauge, histogram, absolute_counter};

use crate::traits::{Monitor, ResultType};

pub struct MetricsExporter {
    monitors: Vec<Box<dyn Monitor + Send + Sync>>,
    wait: u64,
}

impl MetricsExporter {
    pub fn new(wait: u64) -> Self {
        MetricsExporter {
            monitors: Vec::new(),
            wait,
        }
    }

    pub fn add_monitor(&mut self, component: Box<dyn Monitor + Send + Sync>) {
        self.monitors.push(component);
    }

    pub async fn run(&mut self) -> Result<()> {
        let builder = PrometheusBuilder::new();
        builder.install()?;

        loop {
            for monitor in self.monitors.iter_mut() {
                let (result, duration) = monitor.monitor().await?;
                match result {
                    ResultType::Counter(value) => {
                        absolute_counter!(monitor.get_name().to_string(), value);
                        info!(
                            "metric: {}_counter, value: {}, duration: {}ms",
                            monitor.get_name(),
                            value,
                            duration
                        );
                    }
                    ResultType::Gauge(value) => {
                        gauge!(monitor.get_name().to_string(), value);
                        info!(
                            "metric: {}_gauge, value: {}, duration: {}ms",
                            monitor.get_name(),
                            value,
                            duration
                        );
                    }
                    ResultType::Histogram(value) => {
                        histogram!(monitor.get_name().to_string(), value);
                        info!(
                            "metric: {}_histogram, value: {}, duration: {}ms",
                            monitor.get_name(),
                            value,
                            duration
                        );
                    }
                }
                histogram!(format!("{}_duration", monitor.get_name()), duration as f64);
            }
            tokio::time::sleep(Duration::from_secs(self.wait)).await;
        }
    }
}
