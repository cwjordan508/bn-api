use crate::errors::*;
use crate::models::*;
use actix::prelude::*;
use serde_json::Value;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum EventWebsocketType {
    TicketRedemption,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EventWebsocketMessage {
    pub payload: Value,
}

impl EventWebsocketMessage {
    pub fn new(payload: Value) -> Self {
        Self { payload }
    }
}

impl Message for EventWebsocketMessage {
    type Result = Result<(), BigNeonError>;
}

impl Handler<EventWebsocketMessage> for EventWebsocket {
    type Result = Result<(), BigNeonError>;

    fn handle(&mut self, message: EventWebsocketMessage, context: &mut Self::Context) -> Self::Result {
        context.text(serde_json::to_string(&message.payload)?);
        Ok(())
    }
}
