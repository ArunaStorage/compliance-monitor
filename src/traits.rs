use anyhow::Result;
use core::fmt::Debug;

#[allow(dead_code)]
pub enum ResultType {
    Counter(u64),
    Gauge(f64),
    Histogram(f64),
}

#[async_trait::async_trait]
pub trait Monitor {
    // Result<(bool, u64)> is (ResultType, duration_ms)
    async fn monitor(&mut self) -> Result<(ResultType, u64)>;
    fn get_name(&self) -> &str;
}

impl Debug for dyn Monitor + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
