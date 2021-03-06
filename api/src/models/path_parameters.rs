use uuid::Uuid;

#[derive(Deserialize)]
pub struct PathParameters {
    pub id: Uuid,
}

#[derive(Deserialize)]
pub struct OptionalPathParameters {
    pub id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct EventTicketPathParameters {
    pub event_id: Uuid,
    pub ticket_type_id: Uuid,
}
