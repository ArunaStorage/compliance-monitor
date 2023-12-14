use anyhow::Result;
use tracing_subscriber::EnvFilter;
use url::Url;

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

    let mut monitor = monitor::MetricsExporter::new();

    // TLS Monitors 
    dotenvy::var("TLS_CHECK_ENDPOINTS").ok().map(|endpoints| {
        endpoints.split(",").for_each(|endpoint| {
            let name = Url::parse(endpoint).unwrap().host_str().unwrap().to_string();
            monitor.add_monitor(Box::new(monitor_structs::tls::TlsMonitor::new(name, endpoint.to_string()).unwrap()));
        })
    });

    let _ = tokio::spawn(async move {
        monitor.run().await.unwrap();
    }).await?;

    Ok(())
}
