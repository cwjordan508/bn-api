use bigneon_db::dev::TestProject;
use bigneon_db::models::{
    TicketPricing, TicketPricingEditableAttributes, TicketType, TicketTypeEditableAttributes,
};
use bigneon_db::utils::errors::ErrorCode::ValidationError;
use chrono::NaiveDate;
use diesel::result::Error;
use diesel::Connection;

#[test]
fn create() {
    let project = TestProject::new();
    let conn = project.get_connection();
    let event = project.create_event().finish();
    let wallet_id = event.issuer_wallet(conn).unwrap().id;
    let sd = NaiveDate::from_ymd(2016, 7, 8).and_hms(4, 10, 11);
    let ed = NaiveDate::from_ymd(2016, 7, 9).and_hms(4, 10, 11);
    let ticket_type = event
        .add_ticket_type("VIP".to_string(), 100, sd, ed, wallet_id, None, conn)
        .unwrap();

    assert_eq!(ticket_type.event_id, event.id);
    assert_eq!(ticket_type.name, "VIP".to_string())
}

#[test]
fn create_large_amount() {
    let project = TestProject::new();
    let conn = project.get_connection();
    let event = project.create_event().finish();
    let wallet_id = event.issuer_wallet(conn).unwrap().id;
    let sd = NaiveDate::from_ymd(2016, 7, 8).and_hms(4, 10, 11);
    let ed = NaiveDate::from_ymd(2016, 7, 9).and_hms(4, 10, 11);
    let ticket_type = event
        .add_ticket_type("VIP".to_string(), 100_000, sd, ed, wallet_id, None, conn)
        .unwrap();

    assert_eq!(ticket_type.event_id, event.id);
    assert_eq!(ticket_type.name, "VIP".to_string())
}

#[test]
fn validate_record() {
    let project = TestProject::new();
    let event = project.create_event().with_tickets().finish();
    let ticket_type = &event.ticket_types(project.get_connection()).unwrap()[0];
    let start_date1 = NaiveDate::from_ymd(2016, 7, 6).and_hms(4, 10, 11);
    let end_date1 = NaiveDate::from_ymd(2016, 7, 10).and_hms(4, 10, 11);
    let start_date2 = NaiveDate::from_ymd(2016, 7, 7).and_hms(4, 10, 11);
    let end_date2 = NaiveDate::from_ymd(2016, 7, 11).and_hms(4, 10, 11);
    TicketPricing::create(
        ticket_type.id,
        "Early Bird".to_string(),
        start_date1,
        end_date1,
        100,
    ).commit(project.get_connection())
    .unwrap();
    let ticket_pricing = TicketPricing::create(
        ticket_type.id,
        "Regular".to_string(),
        start_date2,
        end_date2,
        100,
    ).commit(project.get_connection())
    .unwrap();
    let mut ticket_pricing_parameters: TicketPricingEditableAttributes = Default::default();

    // Overlapping period
    project
        .get_connection()
        .transaction::<(), Error, _>(|| {
            let validation_results = ticket_type.validate_record(project.get_connection());
            assert!(validation_results.is_err());
            let error = validation_results.unwrap_err();
            match &error.error_code {
                ValidationError { errors } => {
                    assert!(errors.contains_key("ticket_pricing"));
                    assert_eq!(errors["ticket_pricing"].len(), 2);
                    assert_eq!(
                        errors["ticket_pricing"][0].code,
                        "ticket_pricing_overlapping_periods"
                    );
                }
                _ => panic!("Expected validation error"),
            }
            Err(Error::RollbackTransaction)
        }).unwrap_err();

    // Period adjusted to not overlap (after existing record)
    project
        .get_connection()
        .transaction::<(), Error, _>(|| {
            ticket_pricing_parameters.start_date = Some(end_date1);
            ticket_pricing_parameters.end_date =
                Some(NaiveDate::from_ymd(2016, 7, 15).and_hms(4, 10, 11));
            let result =
                ticket_pricing.update(ticket_pricing_parameters.clone(), project.get_connection());
            assert!(result.is_ok());
            let validation_results = ticket_type.validate_record(project.get_connection());
            assert!(validation_results.is_ok());
            Err(Error::RollbackTransaction)
        }).unwrap_err();

    // Period adjusted to not overlap (prior to existing record)
    ticket_pricing_parameters.start_date = Some(NaiveDate::from_ymd(2016, 7, 4).and_hms(4, 10, 11));
    ticket_pricing_parameters.end_date = Some(start_date1);
    let result = ticket_pricing.update(ticket_pricing_parameters.clone(), project.get_connection());
    assert!(result.is_ok());
    let validation_results = ticket_type.validate_record(project.get_connection());
    assert!(validation_results.is_ok());
}

#[test]
pub fn remaining_ticket_count() {
    let project = TestProject::new();
    let connection = project.get_connection();
    let event = project.create_event().with_ticket_pricing().finish();
    let ticket_type = event.ticket_types(connection).unwrap().remove(0);
    let order = project.create_order().for_event(&event).finish();
    assert_eq!(90, ticket_type.remaining_ticket_count(connection).unwrap());

    order.add_tickets(ticket_type.id, 10, connection).unwrap();
    assert_eq!(80, ticket_type.remaining_ticket_count(connection).unwrap());

    let order_item = order.items(connection).unwrap().remove(0);
    assert!(
        order
            .remove_tickets(order_item.ticket_pricing_id.unwrap(), Some(4), connection)
            .is_ok()
    );
    assert_eq!(84, ticket_type.remaining_ticket_count(connection).unwrap());
}

#[test]
fn update() {
    let db = TestProject::new();
    let connection = db.get_connection();
    let event = db.create_event().with_tickets().finish();
    let ticket_type = &event.ticket_types(connection).unwrap()[0];
    //Change editable parameter and submit ticket type update request
    let update_name = String::from("updated_event_name");
    let update_start_date = NaiveDate::from_ymd(2018, 4, 23).and_hms(5, 14, 18);
    let update_end_date = NaiveDate::from_ymd(2018, 6, 1).and_hms(8, 5, 34);
    let update_parameters = TicketTypeEditableAttributes {
        name: Some(update_name.clone()),
        start_date: Some(update_start_date),
        end_date: Some(update_end_date),
        ..Default::default()
    };
    let updated_ticket_type = ticket_type.update(update_parameters, connection).unwrap();
    assert_eq!(updated_ticket_type.id, ticket_type.id);
    assert_eq!(updated_ticket_type.name, update_name);
    assert_eq!(updated_ticket_type.start_date, update_start_date);
    assert_eq!(updated_ticket_type.end_date, update_end_date);
}

#[test]
fn find() {
    let db = TestProject::new();
    let event = db.create_event().with_tickets().finish();
    let ticket_type = &event.ticket_types(&db.get_connection()).unwrap()[0];

    let found_ticket_type = TicketType::find(ticket_type.id, &db.get_connection()).unwrap();
    assert_eq!(&found_ticket_type, ticket_type);
}
