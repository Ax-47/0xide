use crate::infrastructure::external::scylladb;
use scylla::Session;
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
}
