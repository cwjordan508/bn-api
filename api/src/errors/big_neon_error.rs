use actix_web::HttpResponse;
use actix_web::ResponseError;
use bigneon_db::utils::errors::DatabaseError;
use diesel::result::Error as DieselError;
use errors::AuthError;
use errors::*;
use lettre::smtp::error::Error as SmtpError;
use payments::PaymentProcessorError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt;
use tari_client::TariError;

#[derive(Debug)]
pub struct BigNeonError(Box<ConvertToWebError + Send + Sync>);

impl From<DatabaseError> for BigNeonError {
    fn from(e: DatabaseError) -> Self {
        BigNeonError(Box::new(e))
    }
}

impl From<ReqwestError> for BigNeonError {
    fn from(e: ReqwestError) -> Self {
        BigNeonError(Box::new(e))
    }
}

impl From<PaymentProcessorError> for BigNeonError {
    fn from(pe: PaymentProcessorError) -> Self {
        BigNeonError(Box::new(pe))
    }
}

impl From<TariError> for BigNeonError {
    fn from(te: TariError) -> Self {
        BigNeonError(Box::new(te))
    }
}

impl From<DieselError> for BigNeonError {
    fn from(e: DieselError) -> Self {
        BigNeonError(Box::new(e))
    }
}

impl From<SerdeError> for BigNeonError {
    fn from(e: SerdeError) -> Self {
        BigNeonError(Box::new(e))
    }
}

impl From<ApplicationError> for BigNeonError {
    fn from(e: ApplicationError) -> Self {
        BigNeonError(Box::new(e))
    }
}

impl From<SmtpError> for BigNeonError {
    fn from(e: SmtpError) -> Self {
        BigNeonError(Box::new(e))
    }
}

impl From<AuthError> for BigNeonError {
    fn from(e: AuthError) -> Self {
        BigNeonError(Box::new(e))
    }
}

impl fmt::Display for BigNeonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*self.0.to_string())
    }
}

impl Error for BigNeonError {
    fn description(&self) -> &str {
        self.0.description()
    }
}

impl ResponseError for BigNeonError {
    fn error_response(&self) -> HttpResponse {
        self.0.to_response()
    }
}
