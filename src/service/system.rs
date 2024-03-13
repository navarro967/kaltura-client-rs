use crate::KalturaClient;

const SERVICE_PATH: &str = "system";

pub struct SystemService<'client> {
    client: &'client KalturaClient,
}

impl<'client> SystemService<'client> {
    pub fn new(client: &'client KalturaClient) -> SystemService<'client> {
        SystemService { client }
    }

    pub async fn ping(&self) -> String {
        let result = self.client.api_get(SERVICE_PATH, "ping").await.unwrap();
        result
    }

    pub async fn get_time(&self) -> String {
        let result = self.client.api_get(SERVICE_PATH, "getTime").await.unwrap();
        result
    }

    pub async fn get_version(&self) -> String {
        let result = self.client.api_get(SERVICE_PATH, "getVersion").await.unwrap();
        result
    }
}
