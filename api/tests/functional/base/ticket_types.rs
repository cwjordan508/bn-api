use actix_web::{http::StatusCode, FromRequest, HttpResponse, Json, Path, Query};
use bigneon_api::controllers::ticket_types;
use bigneon_api::controllers::ticket_types::*;
use bigneon_api::models::{
    AdminDisplayTicketType, EventTicketPathParameters, PagingParameters, PathParameters, Payload,
};
use bigneon_db::models::*;
use chrono::prelude::*;
use serde_json;
use support;
use support::database::TestDatabase;
use support::test_request::TestRequest;

pub fn create(role: Roles, should_test_succeed: bool) {
    let database = TestDatabase::new();
    let user = database.create_user().finish();
    let organization = database.create_organization().finish();
    let auth_user =
        support::create_auth_user_from_user(&user, role, Some(&organization), &database);
    let event = database
        .create_event()
        .with_organization(&organization)
        .finish();
    //Construct Ticket creation and pricing request
    let test_request = TestRequest::create();
    let state = test_request.extract_state();
    let mut path = Path::<PathParameters>::extract(&test_request.request).unwrap();
    path.id = event.id;
    let mut ticket_pricing: Vec<CreateTicketPricingRequest> = Vec::new();
    let start_date = NaiveDate::from_ymd(2018, 5, 1).and_hms(6, 20, 21);
    let middle_date = NaiveDate::from_ymd(2018, 6, 2).and_hms(7, 45, 31);
    let end_date = NaiveDate::from_ymd(2018, 7, 3).and_hms(9, 23, 23);
    ticket_pricing.push(CreateTicketPricingRequest {
        name: String::from("Early bird"),
        price_in_cents: 10000,
        start_date,
        end_date: middle_date,
    });
    ticket_pricing.push(CreateTicketPricingRequest {
        name: String::from("Base"),
        price_in_cents: 20000,
        start_date: middle_date,
        end_date,
    });
    let request_data = CreateTicketTypeRequest {
        name: "VIP".into(),
        capacity: 1000,
        start_date,
        end_date,
        ticket_pricing,
        increment: None,
    };
    let response: HttpResponse = ticket_types::create((
        database.connection.into(),
        path,
        Json(request_data),
        auth_user,
        state,
    )).into();

    let body = support::unwrap_body_to_string(&response).unwrap();
    if should_test_succeed {
        assert_eq!(response.status(), StatusCode::CREATED);
    } else {
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let temp_json = HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"}));
        let updated_event = support::unwrap_body_to_string(&temp_json).unwrap();
        assert_eq!(body, updated_event);
    }
}

pub fn update(role: Roles, should_test_succeed: bool) {
    let database = TestDatabase::new();
    let request = TestRequest::create();
    let user = database.create_user().finish();
    let organization = database.create_organization().finish();
    let auth_user =
        support::create_auth_user_from_user(&user, role, Some(&organization), &database);
    let event = database
        .create_event()
        .with_organization(&organization)
        .with_tickets()
        .with_ticket_pricing()
        .finish();

    //Retrieve created ticket type and pricing
    let created_ticket_type = &event.ticket_types(&database.connection).unwrap()[0];
    let created_ticket_capacity = created_ticket_type
        .ticket_capacity(&database.connection)
        .unwrap();
    let created_ticket_pricing = created_ticket_type
        .ticket_pricing(&database.connection)
        .unwrap();

    //Construct update request
    let test_request = TestRequest::create_with_uri_event_ticket("/");
    let mut path = Path::<EventTicketPathParameters>::extract(&test_request.request).unwrap();
    path.event_id = event.id;
    path.ticket_type_id = created_ticket_type.id;

    let mut request_ticket_pricing: Vec<UpdateTicketPricingRequest> = Vec::new();
    let start_date = Some(NaiveDate::from_ymd(2018, 5, 1).and_hms(6, 20, 21));
    let middle_date = Some(NaiveDate::from_ymd(2018, 6, 2).and_hms(7, 45, 31));
    let end_date = Some(NaiveDate::from_ymd(2018, 7, 3).and_hms(9, 23, 23));
    let new_pricing_name = String::from("Online");
    //Remove 1st pricing, modify 2nd pricing and add new additional pricing
    request_ticket_pricing.push(UpdateTicketPricingRequest {
        id: Some(created_ticket_pricing[1].id),
        name: Some(String::from("Base")),
        start_date: middle_date,
        end_date,
        price_in_cents: Some(20000),
    });
    request_ticket_pricing.push(UpdateTicketPricingRequest {
        id: None,
        name: Some(new_pricing_name.clone()),
        start_date,
        end_date: middle_date,
        price_in_cents: Some(15000),
    });
    let request_data = UpdateTicketTypeRequest {
        name: Some("Updated VIP".into()),
        capacity: Some(created_ticket_capacity),
        start_date,
        end_date,
        ticket_pricing: Some(request_ticket_pricing),
        increment: None,
    };
    let request_json = serde_json::to_string(&request_data).unwrap();

    //Send update request
    let response: HttpResponse = ticket_types::update((
        database.connection.clone().into(),
        path,
        Json(request_data),
        auth_user,
        request.extract_state(),
    )).into();

    //Check if fields have been updated by retrieving the ticket type and pricing
    let updated_ticket_type = &event.ticket_types(&database.connection).unwrap()[0];
    let updated_ticket_capacity = updated_ticket_type
        .ticket_capacity(&database.connection)
        .unwrap();
    let updated_ticket_pricing = updated_ticket_type
        .ticket_pricing(&database.connection)
        .unwrap();
    let mut new_ticket_pricing: Vec<UpdateTicketPricingRequest> = Vec::new();
    new_ticket_pricing.reserve(updated_ticket_pricing.len());
    for current_ticket_pricing in &updated_ticket_pricing {
        //Replace the id of the new additional pricing with None so we can compare it with the request json
        let option_pricing_id = if current_ticket_pricing.name == new_pricing_name {
            None
        } else {
            Some(current_ticket_pricing.id)
        };

        new_ticket_pricing.push(UpdateTicketPricingRequest {
            id: option_pricing_id,
            name: Some(current_ticket_pricing.name.clone()),
            start_date: Some(current_ticket_pricing.start_date),
            end_date: Some(current_ticket_pricing.end_date),
            price_in_cents: Some(current_ticket_pricing.price_in_cents),
        });
    }
    let updated_data = UpdateTicketTypeRequest {
        name: Some(updated_ticket_type.name.clone()),
        capacity: Some(updated_ticket_capacity),
        start_date: Some(updated_ticket_type.start_date),
        end_date: Some(updated_ticket_type.end_date),
        ticket_pricing: Some(new_ticket_pricing),
        increment: None,
    };
    let updated_json = serde_json::to_string(&updated_data).unwrap();

    if should_test_succeed {
        assert_eq!(request_json, updated_json);
        assert_eq!(response.status(), StatusCode::OK);
    } else {
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}

pub fn index(role: Roles, should_test_succeed: bool) {
    let database = TestDatabase::new();
    let user = database.create_user().finish();
    let request = TestRequest::create();
    let organization = database.create_organization().finish();
    let fee_schedule =
        FeeSchedule::find(organization.fee_schedule_id, &database.connection).unwrap();
    let auth_user =
        support::create_auth_user_from_user(&user, role, Some(&organization), &database);

    let event = database
        .create_event()
        .with_organization(&organization)
        .with_ticket_pricing()
        .finish();

    let mut path = Path::<PathParameters>::extract(&request.request).unwrap();
    path.id = event.id;
    let test_request = TestRequest::create_with_uri(&format!("/limits?"));
    let query_parameters =
        Query::<PagingParameters>::from_request(&test_request.request, &()).unwrap();
    let response = ticket_types::index((
        database.connection.clone().into(),
        path,
        query_parameters,
        auth_user,
    )).unwrap();
    if should_test_succeed {
        let body = support::unwrap_body_to_string(&response).unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let ticket_type = &event.ticket_types(&database.connection).unwrap()[0];
        let expected_ticket_types = vec![
            AdminDisplayTicketType::from_ticket_type(
                ticket_type,
                &fee_schedule,
                &database.connection,
            ).unwrap(),
        ];
        let ticket_types_response: Payload<AdminDisplayTicketType> =
            serde_json::from_str(&body).unwrap();
        assert_eq!(ticket_types_response.data, expected_ticket_types);
    } else {
        support::expects_unauthorized(&response);
    }
}
