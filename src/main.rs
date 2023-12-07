use std::sync::Arc;

use anyhow::Result;
use monitor::MonitorRunner;
use monitor_structs::endpoints::EndpointMonitor;
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod monitor;
mod monitor_structs;
mod traits;

#[tokio::main]
async fn main() -> Result<()> {
    /*
     * Service Loop:
     *  - Requests against all DataProxies
     *   1. GetCredentialsRequest with secure and unsecure channel
     *
     *  - Get Requests against sentinel resources
     *   1. Permissions
     *   2. Consistency
     *   3. Data usage agreement
     *
     *  - Check container signatures?
     *    Maybe it is easiert to just enforce directly inside the Kubernetes cluster?
     *   1. Cosign + ???
     *
     *  - EULA/TOS before user activation
     * 
     * - Deliver (static?) monitoring results page
     */

    // Read .env variables
    dotenvy::from_filename(".env").ok();

    // Init tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or("none".into())
        .add_directive("compliance_monitor=trace".parse()?);

    let subscriber = tracing_subscriber::fmt()
        //.with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        // Use a more compact, abbreviated log format
        .compact()
        // Set LOG_LEVEL to
        .with_env_filter(filter)
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        .with_target(false)
        .finish();

    todo!();
}
