use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::blocking::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost:8080/engine-rest".to_owned(),
            user_agent: Some("Beakwise-Beaksurance-Camunda-Client/rust/1.0".to_owned()),
            client: reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(90))
                .connect_timeout(Duration::from_secs(10))
                // .https_only(true) //TODO must be activated for production
                .pool_idle_timeout(Duration::from_secs(60000))
                .tcp_keepalive(Duration::from_secs(6000))
                .pool_max_idle_per_host(100)
                .build()
                .expect("Cannot build a respwest::Client!"),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
