use anyhow::Result;
use chrono::NaiveDateTime;
use diesel_ulid::DieselUlid;
use postgres_types::Json;
use std::sync::Arc;

use crate::{
    database::{
        connection::Database,
        monitor_dsl::{MonitorResult, MonitorResultType, MonitorType, RunParameters},
    },
    traits::Monitor,
};

pub enum Credentials<'a> {
    S3(S3Credentials<'a>),
    Token(&'a str),
}

pub struct S3Credentials<'a> {
    access_key_id: &'a str,
    access_secret: &'a str,
}

pub struct EndpointMonitor<'a> {
    database: Arc<Database>,
    endpoint_host: &'a str,
    credentials: Credentials<'a>,
    last_monitor: NaiveDateTime,
    current_state: MonitorResultType,
}

#[async_trait::async_trait]
impl<'a> Monitor for EndpointMonitor<'a> {
    async fn monitor(self: &mut EndpointMonitor<'a>) -> Result<bool> {
        //Something, something requests to endpoint

        // Update self
        self.update_last_monitor(chrono::Utc::now().naive_local(), todo!())
            .await?;

        Ok(true)
    }
}

impl<'a> EndpointMonitor<'a> {
    pub fn new(host: &'a str, database: Arc<Database>) -> Result<Self> {
        Ok(EndpointMonitor {
            database,
            credentials: Credentials::S3(S3Credentials {
                access_key_id: "",
                access_secret: "",
            }),
            endpoint_host: host,
            last_monitor: chrono::Utc::now().naive_utc(),
            current_state: MonitorResultType::FAIL, // Default is pessimistic
        })
    }

    async fn update_last_monitor(
        &mut self,
        timestamp: NaiveDateTime,
        monitor_result: MonitorResultType,
    ) -> Result<()> {
        // Update self
        self.last_monitor = timestamp;
        self.current_state = monitor_result;

        // Update persistence
        let client = self.database.get_client().await?;

        let mut insert = MonitorResult {
            id: DieselUlid::generate(),
            monitor_type: MonitorType::ENDPOINT_SECURITY,
            monitor_result,
            run_parameters: Json(RunParameters {}),
        };

        insert.create(&client).await?;

        Ok(())
    }
}
