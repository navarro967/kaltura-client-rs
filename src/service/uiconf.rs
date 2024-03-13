use crate::KalturaClient;

const SERVICE_PATH: &str = "uiconf";

pub struct UiConfService<'client> {
    client: &'client KalturaClient,
}

impl<'client> UiConfService<'client> {
    pub fn new(client: &'client KalturaClient) -> UiConfService<'client> {
        UiConfService { client }
    }
}
