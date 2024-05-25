use crate::infrastructure::external::scylladb;
use scylla::transport::errors::QueryError;
use scylla::transport::query_result;
use scylla::{QueryResult, Session};
use std::error::Error;
#[derive(Debug)]
pub struct GuildService {
    pub scylla_session: Session,
}
impl GuildService {
    pub async fn new(uri: String) -> Result<GuildService, Box<dyn Error>> {
        let scylla_session: Session = scylladb::connect_scylladb(uri).await?;
        Ok(GuildService { scylla_session })
    }
    pub async fn create_guild(&self, guild_id: String) -> Result<QueryResult, QueryError> {
        let query_result = self
            .scylla_session
            .query("INSERT INTO oxide.guild (id) VALUES(?)", (guild_id,))
            .await?;
        Ok(query_result)
    }
    pub async fn update_join_channel(
        &self,
        guild_id: String,
        channel_id: String,
    ) -> Result<QueryResult, QueryError> {
        let query_result = self
            .scylla_session
            .query(
                "UPDATE oxide.guild SET join_channel = ? WHERE id = ?",
                (channel_id, guild_id),
            )
            .await?;
        Ok(query_result)
    }
}
