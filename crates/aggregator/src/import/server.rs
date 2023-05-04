use crate::db::ServerData;
use crate::import::errors::Error;
use universalis_sdk::get_servers;

pub struct ServerImport {
    server_data: ServerData,
}
impl ServerImport {
    pub fn new(server_data: ServerData) -> Self {
        Self { server_data }
    }
    pub async fn import_servers(&self) -> Result<(), Error> {
        let servers = get_servers().await?;
        self.server_data.save_servers(servers).await?;
        Ok(())
    }
}
