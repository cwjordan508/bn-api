use dotenv::dotenv;
use mail::transports::{SmtpTransport, TestTransport, Transport};
use std::env;
use tari_client::{HttpTariClient, TariClient, TariTestClient};

#[derive(Clone, PartialEq)]
pub enum Environment {
    Development,
    Test,
    Production,
}

#[derive(Clone)]
pub struct Config {
    pub allowed_origins: String,
    pub front_end_url: String,
    pub api_url: String,
    pub api_port: String,
    pub app_name: String,
    pub database_url: String,
    pub domain: String,
    pub environment: Environment,
    pub facebook_app_id: Option<String>,
    pub facebook_app_secret: Option<String>,
    pub google_recaptcha_secret_key: Option<String>,
    pub mail_from_email: String,
    pub mail_from_name: String,
    pub mail_transport: Box<Transport + Send + Sync>,
    pub primary_currency: String,
    pub stripe_secret_key: String,
    pub token_secret: String,
    pub token_issuer: String,
    pub tari_client: Box<TariClient + Send + Sync>,
}

const ALLOWED_ORIGINS: &str = "ALLOWED_ORIGINS";
const APP_NAME: &str = "APP_NAME";
const API_URL: &str = "API_URL";
const API_PORT: &str = "API_PORT";
const DATABASE_URL: &str = "DATABASE_URL";
const DOMAIN: &str = "DOMAIN";
const FACEBOOK_APP_ID: &str = "FACEBOOK_APP_ID";
const FACEBOOK_APP_SECRET: &str = "FACEBOOK_APP_SECRET";
const GOOGLE_RECAPTCHA_SECRET_KEY: &str = "GOOGLE_RECAPTCHA_SECRET_KEY";
const PRIMARY_CURRENCY: &str = "PRIMARY_CURRENCY";
const STRIPE_SECRET_KEY: &str = "STRIPE_SECRET_KEY";
const TARI_URL: &str = "TARI_URL";
const TEST_DATABASE_URL: &str = "TEST_DATABASE_URL";
const TOKEN_SECRET: &str = "TOKEN_SECRET";
const TOKEN_ISSUER: &str = "TOKEN_ISSUER";

// Mail settings
const MAIL_FROM_EMAIL: &str = "MAIL_FROM_EMAIL";
const MAIL_FROM_NAME: &str = "MAIL_FROM_NAME";
// Optional for test environment, required for other environments
const MAIL_SMTP_HOST: &str = "MAIL_SMTP_HOST";
const MAIL_SMTP_PORT: &str = "MAIL_SMTP_PORT";
const FRONT_END_URL: &str = "FRONT_END_URL";

impl Config {
    pub fn new(environment: Environment) -> Self {
        dotenv().ok();

        let app_name = env::var(&APP_NAME).unwrap_or_else(|_| "Big Neon".to_string());

        let database_url = match environment {
            Environment::Test => env::var(&TEST_DATABASE_URL)
                .unwrap_or_else(|_| panic!("{} must be defined.", DATABASE_URL)),
            _ => env::var(&DATABASE_URL)
                .unwrap_or_else(|_| panic!("{} must be defined.", DATABASE_URL)),
        };
        let domain = env::var(&DOMAIN).unwrap_or_else(|_| "api.bigneon.com".to_string());
        let mail_from_email = env::var(&MAIL_FROM_EMAIL)
            .unwrap_or_else(|_| panic!("{} must be defined.", MAIL_FROM_EMAIL));
        let mail_from_name = env::var(&MAIL_FROM_NAME)
            .unwrap_or_else(|_| panic!("{} must be defined.", MAIL_FROM_NAME));

        let mail_transport = match environment {
            Environment::Test => Box::new(TestTransport::new()) as Box<Transport + Send + Sync>,
            _ => {
                let host = env::var(&MAIL_SMTP_HOST)
                    .unwrap_or_else(|_| panic!("{} must be defined.", MAIL_SMTP_HOST));

                if host == "test" {
                    Box::new(TestTransport::new()) as Box<Transport + Send + Sync>
                } else {
                    let port = env::var(&MAIL_SMTP_PORT)
                        .unwrap_or_else(|_| panic!("{} must be defined.", MAIL_SMTP_PORT));

                    info!("Mail configured {}:{}", host, port);

                    Box::new(SmtpTransport::new(
                        &domain,
                        &host,
                        port.parse::<u16>().unwrap(),
                    )) as Box<Transport + Send + Sync>
                }
            }
        };

        let allowed_origins = env::var(&ALLOWED_ORIGINS).unwrap_or_else(|_| "*".to_string());
        let api_url = env::var(&API_URL).unwrap_or_else(|_| "127.0.0.1".to_string());
        let api_port = env::var(&API_PORT).unwrap_or_else(|_| "8088".to_string());

        let primary_currency = env::var(&PRIMARY_CURRENCY).unwrap_or_else(|_| "usd".to_string());
        let stripe_secret_key =
            env::var(&STRIPE_SECRET_KEY).unwrap_or_else(|_| "<stripe not enabled>".to_string());
        let token_secret =
            env::var(&TOKEN_SECRET).unwrap_or_else(|_| panic!("{} must be defined.", TOKEN_SECRET));

        let token_issuer =
            env::var(&TOKEN_ISSUER).unwrap_or_else(|_| panic!("{} must be defined.", TOKEN_ISSUER));

        let facebook_app_id = env::var(&FACEBOOK_APP_ID).ok();

        let facebook_app_secret = env::var(&FACEBOOK_APP_SECRET).ok();

        let front_end_url =
            env::var(&FRONT_END_URL).unwrap_or_else(|_| panic!("Front end url must be defined"));

        let tari_uri =
            env::var(&TARI_URL).unwrap_or_else(|_| panic!("{} must be defined.", TARI_URL));

        let tari_client = match environment {
            Environment::Test => {
                Box::new(TariTestClient::new(tari_uri)) as Box<TariClient + Send + Sync>
            }
            _ => if tari_uri == "TEST" {
                Box::new(TariTestClient::new(tari_uri)) as Box<TariClient + Send + Sync>
            } else {
                Box::new(HttpTariClient::new(tari_uri)) as Box<TariClient + Send + Sync>
            },
        };

        let google_recaptcha_secret_key = env::var(&GOOGLE_RECAPTCHA_SECRET_KEY).ok();

        Config {
            allowed_origins,
            app_name,
            api_url,
            api_port,
            database_url,
            domain,
            environment,
            facebook_app_id,
            facebook_app_secret,
            google_recaptcha_secret_key,
            mail_from_name,
            mail_from_email,
            mail_transport,
            primary_currency,
            stripe_secret_key,
            token_secret,
            token_issuer,
            front_end_url,
            tari_client,
        }
    }
}
