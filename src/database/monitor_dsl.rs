use anyhow::Result;
use diesel_ulid::DieselUlid;
use postgres_from_row::FromRow;
use postgres_types::{FromSql, Json, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, ToSql, Eq, PartialOrd, FromSql)]
pub enum MonitorResultType {
    SUCCESS,
    FAIL,
    ERROR,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, ToSql, Eq, PartialOrd, FromSql)]
pub enum MonitorType {
    ENDPOINT_SECURITY,
    POLICY_EVALUATION,
    DATA_PRIVACY,
    CONTAINER_SECURITY,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunParameters {
    //ToDo:
}

#[derive(FromRow, Debug, PartialEq)]
pub struct MonitorResult {
    pub id: DieselUlid,
    pub monitor_type: MonitorType,
    pub monitor_result: MonitorResultType,
    pub run_parameters: Json<RunParameters>,
}

impl MonitorResult {
    pub async fn create(&mut self, client: &Client) -> Result<MonitorResult> {
        let query = "INSERT INTO monitor_results 
            (id, monitor_type, monitor_result, run_parameters) 
              VALUES 
            ($1, $2, $3, $4)
              RETURNING *;";
        let prepared = client.prepare(query).await?;

        let row = client
            .query_one(
                &prepared,
                &[
                    &self.id,
                    &self.monitor_type,
                    &self.monitor_result,
                    &self.run_parameters,
                ],
            )
            .await?;

        Ok(MonitorResult::from_row(&row))
    }
}
