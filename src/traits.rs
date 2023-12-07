use anyhow::Result;
use core::fmt::Debug;

#[async_trait::async_trait]
pub trait Monitor {
    // Result<(bool, u64)> is (is_ok, duration_ms)
    async fn monitor(&mut self) -> Result<(bool, u64)>;
}

impl Debug for dyn Monitor + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
