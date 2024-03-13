pub mod crypto;
pub mod models;
mod service;

use base64::{engine::general_purpose::URL_SAFE, Engine};
use rand;
use std::{collections::HashMap, time};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const KALTURA_API_ENDPOINT: &str = "https://www.kaltura.com/api_v3";
const USER_AGENT: &str = "kaltura-client-rs";

#[derive(Debug, PartialEq)]
pub struct KalturaClientConfig {
    pub service_url: Option<String>,
}

impl Default for KalturaClientConfig {
    fn default() -> Self {
        KalturaClientConfig {
            service_url: Some(KALTURA_API_ENDPOINT.to_string()),
        }
    }
}

/// The `KalturaClient` struct represents a client for the Kaltura API.
///
/// It includes configuration, an HTTP client, headers, and a session.
///
/// # Example
///
/// ```
/// use kaltura_client_rs::KalturaClient;
/// let client = KalturaClient::new();
/// ```
#[derive(Default, Debug)]
pub struct KalturaClient {
    config: KalturaClientConfig,
    http_client: reqwest::Client,
    headers: HashMap<String, String>,
    session: models::session::KalturaSession,
}

impl KalturaClient {
    /// Creates a new `KalturaClient`.
    ///
    /// # Example
    ///
    /// ```
    /// use kaltura_client_rs::KalturaClient;
    /// let client = KalturaClient::new();
    /// ```
    pub fn new() -> KalturaClient {
        let user_agent: String = format!("{}/{}", USER_AGENT, VERSION);
        let headers: HashMap<String, String> = HashMap::from([
            (reqwest::header::USER_AGENT.to_string(), user_agent),
            (
                reqwest::header::CONTENT_TYPE.to_string(),
                String::from("application/json"),
            ),
        ]);

        KalturaClient {
            config: KalturaClientConfig::default(),
            http_client: reqwest::Client::new(),
            session: models::session::KalturaSession::default(),
            headers,
        }
    }
    
    /// Returns a `KalturaClientBuilder` for creating a `KalturaClient` with custom configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use kaltura_client_rs::KalturaClient;
    /// let builder = KalturaClient::builder();
    /// ```
    pub fn builder() -> KalturaClientBuilder {
        KalturaClientBuilder::default()
    }
    
    /// Returns a `SystemService` for interacting with the system service of the Kaltura API.
    ///
    /// # Example
    ///
    /// ```
    /// use kaltura_client_rs::KalturaClient;
    /// let kaltura_client = KalturaClient::new();
    /// tokio_test::block_on(async {
    ///     let response =  kaltura_client.system()
    ///         .get_version().await;
    ///    assert!(response.len() > 0);
    /// });
    ///     
    /// ```
    pub fn system(&self) -> service::system::SystemService {
        service::system::SystemService::new(self)
    }
    /// Sends a GET request to the specified URL and returns the response as a `String`.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to send the GET request to.
    ///
    /// # Example
    ///
    /// ```
    /// use kaltura_client_rs::KalturaClient;
    /// 
    /// let client = KalturaClient::new();
    /// 
    /// tokio_test::block_on(async {
    ///     let response = client.get("/").await;
    ///    assert!(response.is_ok());
    /// });
    /// ```
    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let request: String = format!(
            "{}/{}",
            self.config.service_url.as_ref().unwrap_or(&KALTURA_API_ENDPOINT.to_string()),
            url
        );
        let headers = (&self.headers)
            .try_into()
            .expect("Error converting headers");
        let resp = self.http_client
            .get(request)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }

    /// Sends a GET request to the specified URL with the specified body and returns the response as a `String`.
    ///
    /// # Arguments
    /// * `url` - The URL to send the POST request to.
    /// * `body` - The body of the POST request.
    /// 
    /// # Example
    /// 
    /// ```
    /// use kaltura_client_rs::KalturaClient;
    /// 
    /// let client = KalturaClient::new();
    /// 
    /// tokio_test::block_on(async {
    ///    let response = client.api_get("system", "ping").await;
    ///   assert!(response.is_ok());
    /// });
    /// ```
    pub async fn api_get(&self, service: &str, action: &str) -> Result<String, reqwest::Error> {
        let request: String = format!(
            "/service/{}/action/{}",
            service, action
        );
        self.get(&request).await
    }
    // pub async fn api_get<T: Serialize + Deserialize>(&self, service: &str, action: &str, parameters: T) -> Result<String, reqwest::Error> {
    //     let request: String = format!(
    //         "/service/{}/action/{}",
    //         service, action
    //     );
    //     self.get(&request).await
    // }
}

#[derive(Default)]
pub struct KalturaClientBuilder {
    pub client: KalturaClient,
}

impl KalturaClientBuilder {
    pub fn new() -> KalturaClientBuilder {
        KalturaClientBuilder {
            client: KalturaClient::new(),
        }
    }
    pub fn with_admin_secret<'a>(mut self, admin_secret: &'a str) -> KalturaClientBuilder {
        self.client.session.secret = admin_secret.to_string();
        self
    }
    pub fn with_service_url<'a>(mut self, service_url: &'a str) -> KalturaClientBuilder {
        self.client.config.service_url = Some(service_url.to_string());
        self
    }
    pub fn with_user_id<'a>(mut self, user_id: &'a str) -> KalturaClientBuilder {
        self.client.session.user_id = user_id.to_string();
        self
    }
    pub fn with_partner_id<'a>(mut self, partner_id: &'a i32) -> KalturaClientBuilder {
        self.client.session.partner_id = *partner_id;
        self
    }
    pub fn with_permissions<'a>(mut self, permissions: &'a str) -> KalturaClientBuilder {
        self.client.session.privileges = permissions.to_string();
        self
    }
    pub fn with_ks<'a>(mut self, ks: &'a str) -> KalturaClientBuilder {
        self.client.session.ks = ks.to_string();
        self
    }
    pub fn build(mut self) -> KalturaClient {
        if self.client.session.ks == "" && self.client.session.secret.len() > 0 {
            generate_session(&mut self.client.session);
        }
        self.client
    }
}

/// This function generates a new Kaltura Session for a given user.
///
/// # Arguments
///
/// * `session` - A mutable reference to a KalturaSession object.
///
/// # Example
///
/// ```
/// use kaltura_client_rs::{KalturaClient, generate_session, models::session::{KalturaSession, SessionType}};
/// let mut ks = KalturaSession::new(
///     "secret".to_string(),
///     "joshua.navarro@kaltura.com".to_string(),
///     4414853,
///     3600,
///     "disableentitlement".to_string(),
///     SessionType::USER,
/// );
/// generate_session(&mut ks);
/// println!("{}", ks.ks);
/// assert!(ks.ks.len() > 0);
/// ```
pub fn generate_session(session: &mut models::session::KalturaSession) {
    let session_data = format!(
        "{};{};{};{};{:.4};{};{};;",
        session.partner_id,
        session.partner_id,
        session_duration(session.expiry as i32),
        0,
        session_duration(session.expiry),
        session.user_id,
        session.privileges,
    );
    let session_hash = crypto::sha1(&format!("{}{}", session.secret, session_data).into_bytes());
    session.ks = URL_SAFE.encode(format!(
        "{}{}",
        session_hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>(),
        session_data
    ));
}
/// This function generates a new Kaltura Session V2 for a given user.
///
/// # Arguments
///
/// * `session` - A mutable reference to a KalturaSession object.
///
/// # Example
///
/// ```
/// use kaltura_client_rs::models::session::{KalturaSession, SessionType};
/// use kaltura_client_rs::generate_session_v2;
///
/// let mut session = KalturaSession {
///     expiry: 3600,
///     user_id: "test_user".to_string(),
///     secret: "test_secret".to_string(),
///     partner_id: 0,
/// 
///     privileges: "disableentitlement".to_string(),
///     session_type: SessionType::ADMIN,
///     ks: "".to_string(),
/// };
///
/// generate_session_v2(&mut session);
///
/// assert!(session.ks.len() > 0);
/// ```
pub fn generate_session_v2(session: &mut models::session::KalturaSession) {
    let session_duration = session_duration(session.expiry).to_string();
    let mut session_data: HashMap<&str, &str> = HashMap::new();
    session_data.insert("_e", &session_duration);
    session_data.insert("_u", &session.user_id);
    session_data.insert("_t", "0");
    session_data.extend(generate_privileges(session));

    let mut buffer: Vec<u8> = (0..crypto::AES_KEY_LEN)
        .map(|_| rand::Rng::gen_range(&mut rand::thread_rng(), 65..126))
        .collect();
    buffer.extend(serde_qs::to_string(&session_data).unwrap().into_bytes());

    crypto::sha1(&buffer)
        .iter()
        .rev()
        .for_each(|b| buffer.insert(0, *b));

    crypto::aes_encrypt(
        &mut buffer,
        &crypto::sha1(&session.secret.as_bytes().to_vec()),
        &crypto::AES_IV,
    );

    let mut ks: Vec<u8> = format!("v2|{}|", session.partner_id).into_bytes();
    ks.append(&mut buffer.to_vec());
    session.ks = URL_SAFE.encode(ks);
}

fn generate_privileges(session: &models::session::KalturaSession) -> HashMap<&str, &str> {
    session
        .privileges
        .split(",")
        .map(|privilege| match privilege.trim() {
            "*" => ("all", "*"),
            line if line.contains(":") => {
                let mut parts = line.split(":");
                (parts.next().unwrap(), parts.next().unwrap())
            }
            _ => (privilege, ""),
        })
        .collect()
}

fn session_duration(duration: i32) -> f32 {
    let expiry: f32 = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f32()
        + duration as f32;
    expiry
}

#[tokio::test]
async fn system_service_test() {
    let kaltura_client: KalturaClient = KalturaClient::builder()
        .with_service_url("https://api.nvq2.ovp.kaltura.com/api_v3")
        .with_admin_secret("secret")
        .with_partner_id(&102)
        .with_user_id("onprem-alerts@kaltura.com")
        .build();

    let result = kaltura_client.system().get_version().await;
    if kaltura_client.session.ks.len() > 0 {
        println!("Ks: {:?}", kaltura_client.session.ks);
    }
    println!("result: {:?}", result);
    assert_eq!(result.len() > 0, true);
}
