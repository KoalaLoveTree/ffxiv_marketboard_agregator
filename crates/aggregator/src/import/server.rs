use crate::import::errors::Error;
use db_sdk::ServerData;
use universalis_sdk::get_servers;

pub struct ServerImport<'a> {
    server_data: &'a ServerData,
}
impl<'a> ServerImport<'a> {
    pub fn new(server_data: &'a ServerData) -> Self {
        Self { server_data }
    }
    pub async fn import_servers(&self) -> Result<(), Error> {
        let servers = get_servers().await?;
        self.server_data.save_servers(servers).await?;
        Ok(())
    }
}
