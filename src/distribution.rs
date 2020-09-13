use crate::errors::Error;
use crate::Result;
use twilio::{Client, OutboundMessage, TwilioError};

pub trait Distributor {
    fn distribute(&self, to: &str, message: &str) -> Result<()>;
}

// Todo: better twilio error handling
impl From<TwilioError> for Error {
    fn from(err: TwilioError) -> Self {
        match err {
            TwilioError::AuthError => Error(String::from("twilio auth error")),
            TwilioError::NetworkError => Error(String::from("twilio network error")),
            TwilioError::HTTPError => Error(String::from("twilio HTTP error")),
            TwilioError::ParsingError => Error(String::from("twilio parsing error")),
            TwilioError::BadRequest => Error(String::from("twilio bad request")),
        }
    }
}

pub struct TwilioDistributor {
    client: Client,
    from_number: String,
}

impl TwilioDistributor {
    pub fn new(account_id: String, auth_token: String, from_number: String) -> TwilioDistributor {
        let client = Client::new(&account_id, &auth_token);

        return TwilioDistributor {
            client,
            from_number,
        };
    }
}

impl Distributor for TwilioDistributor {
    fn distribute(&self, to: &str, message: &str) -> Result<()> {
        let msg = OutboundMessage::new(&self.from_number, to, message);
        self.client.send_message(msg)?;

        Ok(())
    }
}

pub struct MockDistributor {}

impl Distributor for MockDistributor {
    // yo this shit is mock
    fn distribute(&self, _: &str, _: &str) -> Result<()> {
        Ok(())
    }
}
