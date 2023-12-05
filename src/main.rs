use std::sync::Arc;

use anyhow::Result;
use monitor::MonitorRunner;
use monitor_structs::endpoints::EndpointMonitor;
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod database;
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

    tracing::subscriber::set_global_default(subscriber)?;

    // Init database connection
    let database = database::connection::Database::new(
        dotenvy::var("DATABASE_HOST")?,
        dotenvy::var("DATABASE_PORT")?.parse::<u16>()?,
        dotenvy::var("DATABASE_NAME")?,
        dotenvy::var("DATABASE_USER")?,
        dotenvy::var("DATABASE_PASSWORD")?,
    )?;
    database.initialize_db().await?;
    let database_arc = Arc::new(database);

    // Read monitoring interval from config or use default of 1 hour
    let monitor_interval = dotenvy::var("MONITOR_INTERVAL")
        .unwrap_or("3600".to_string())
        .parse::<u64>()?;

    //ToDo: Replace dummy monitoring structs init
    let mut monitors = MonitorRunner::new();

    monitors.add_component(Arc::new(Mutex::new(Box::new(EndpointMonitor::new(
        "https://proxy.gi.dev.aruna-storage.org",
        database_arc.clone(),
    )?))));

    monitors.add_component(Arc::new(Mutex::new(Box::new(EndpointMonitor::new(
        "https://proxy.bi.dev.aruna-storage.org",
        database_arc.clone(),
    )?))));

    monitors.add_component(Arc::new(Mutex::new(Box::new(EndpointMonitor::new(
        "https://proxy.be.dev.aruna-storage.org",
        database_arc.clone(),
    )?))));

    // Loop until the end of time
    info!("Starting monitor loop.");
    let mut counter = 0;
    loop {
        counter = counter + 1;
        info!("Monitor loop iteration: {}", counter);

        if let Err(err) = monitors.run().await {
            error!("Monitor run init failed: {}", err)
        };

        // Sleep for 1 hour.
        std::thread::sleep(std::time::Duration::from_secs(monitor_interval))
    }
}
