use scylla::{Session, SessionBuilder};
use std::error::Error;
pub async fn connect_scylladb(uri: String) -> Result<Session, Box<dyn Error>> {
    let session: Session = SessionBuilder::new().known_node(uri).build().await?;
    Ok(session)
}
