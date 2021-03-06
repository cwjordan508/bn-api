use actix_web::middleware::cors::CorsBuilder;
use actix_web::{http::Method, App, HttpResponse};
use controllers::*;
use server::AppState;

pub fn routes(app: &mut CorsBuilder<AppState>) -> App<AppState> {
    // Please try to keep in alphabetical order
    app.resource("/artists/{id}/toggle_privacy", |r| {
        r.method(Method::PUT).with(artists::toggle_privacy);
    }).resource("/artists/{id}", |r| {
        r.method(Method::GET).with(artists::show);
        r.method(Method::PUT).with(artists::update);
    }).resource("/artists", |r| {
        r.method(Method::GET).with(artists::index);
        r.method(Method::POST).with(artists::create);
    }).resource("/auth/token", |r| r.method(Method::POST).with(auth::token))
    .resource("/auth/token/refresh", |r| {
        r.method(Method::POST).with(auth::token_refresh)
    }).resource("/cart", |r| {
        r.method(Method::POST).with(cart::add);
        r.method(Method::GET).with(cart::show);
        r.method(Method::DELETE).with(cart::remove);
    }).resource("/cart/checkout", |r| {
        r.method(Method::POST).with(cart::checkout);
    }).resource("/cart/{id}", |r| {
        r.method(Method::GET).with(cart::show);
    }).resource("/events", |r| {
        r.method(Method::GET).with(events::index);
        r.method(Method::POST).with(events::create);
    }).resource("/events/{id}", |r| {
        r.method(Method::GET).with(events::show);
        r.method(Method::PUT).with(events::update);
        r.method(Method::DELETE).with(events::cancel);
    }).resource("/events/{id}/artists", |r| {
        r.method(Method::POST).with(events::add_artist);
        r.method(Method::PUT).with(events::update_artists);
    }).resource("/events/{id}/guests", |r| {
        r.method(Method::GET).with(events::guest_list);
    }).resource("/events/{id}/interest", |r| {
        r.method(Method::GET).with(events::list_interested_users);
        r.method(Method::POST).with(events::add_interest);
        r.method(Method::DELETE).with(events::remove_interest);
    }).resource("/events/{id}/publish", |r| {
        r.method(Method::POST).with(events::publish);
    }).resource("/events/{id}/tickets", |r| {
        r.method(Method::GET).with(tickets::index);
    }).resource("/events/{id}/ticket_types", |r| {
        r.method(Method::GET).with(ticket_types::index);
        r.method(Method::POST).with(ticket_types::create);
    }).resource("/events/{event_id}/ticket_types/{ticket_type_id}", |r| {
        r.method(Method::PATCH).with(ticket_types::update);
    }).resource("/events/{id}/holds", |r| {
        r.method(Method::POST).with(holds::create);
    }).resource("/external/facebook/web_login", |r| {
        r.method(Method::POST).with(external::facebook::web_login)
    }).resource("/invitations/{id}", |r| {
        r.method(Method::GET).with(organization_invites::view);
    }).resource("/invitations", |r| {
        r.method(Method::POST)
            .with(organization_invites::accept_request);
        r.method(Method::DELETE)
            .with(organization_invites::decline_request);
    }).resource("/holds/{id}/tickets", |r| {
        r.method(Method::PUT).with(holds::add_remove_from_hold);
    }).resource("/holds/{id}", |r| {
        r.method(Method::PATCH).with(holds::update);
    }).resource("/orders", |r| {
        r.method(Method::GET).with(orders::index);
    }).resource("/orders/{id}", |r| {
        r.method(Method::GET).with(orders::show);
    }).resource("/organizations/{id}/artists", |r| {
        r.method(Method::GET).with(artists::show_from_organizations);
        r.method(Method::POST).with(organizations::add_artist);
    }).resource("/organizations/{id}/events", |r| {
        r.method(Method::GET).with(events::show_from_organizations);
    }).resource("/organizations/{id}/fee_schedule", |r| {
        r.method(Method::GET).with(organizations::show_fee_schedule);
        r.method(Method::POST).with(organizations::add_fee_schedule);
    }).resource("/organizations/{id}/invite", |r| {
        r.method(Method::POST).with(organization_invites::create);
    }).resource("/organizations/{id}/owner", |r| {
        r.method(Method::PUT).with(organizations::update_owner);
    }).resource("/organizations/{id}/users", |r| {
        r.method(Method::POST).with(organizations::add_user);
        r.method(Method::DELETE).with(organizations::remove_user);
        r.method(Method::GET)
            .with(organizations::list_organization_members);
    }).resource("/organizations/{id}/venues", |r| {
        r.method(Method::GET).with(venues::show_from_organizations);
        r.method(Method::POST).with(organizations::add_venue);
    }).resource("/organizations/{id}", |r| {
        r.method(Method::GET).with(organizations::show);
        r.method(Method::PATCH).with(organizations::update);
    }).resource("/organizations", |r| {
        r.method(Method::GET).with(organizations::index);
        r.method(Method::POST).with(organizations::create);
    }).resource("/password_reset", |r| {
        r.method(Method::POST).with(password_resets::create);
        r.method(Method::PUT).with(password_resets::update);
    }).resource("/payment_methods", |r| {
        r.method(Method::GET).with(payment_methods::index);
    }).resource("/regions/{id}", |r| {
        r.method(Method::GET).with(regions::show);
        r.method(Method::PUT).with(regions::update);
    }).resource("/regions", |r| {
        r.method(Method::GET).with(regions::index);
        r.method(Method::POST).with(regions::create)
    }).resource("/status", |r| {
        r.method(Method::GET).f(|_| HttpResponse::Ok())
    }).resource("/tickets/transfer", |r| {
        r.method(Method::POST).with(tickets::transfer_authorization);
    }).resource("/tickets/receive", |r| {
        r.method(Method::POST).with(tickets::receive_transfer);
    }).resource("/tickets/send", |r| {
        r.method(Method::POST).with(tickets::send_via_email);
    }).resource("/tickets/{id}", |r| {
        r.method(Method::GET).with(tickets::show);
    }).resource("/tickets", |r| {
        r.method(Method::GET).with(tickets::index);
    }).resource("/tickets/{id}/redeem", |r| {
        r.method(Method::GET).with(tickets::show_redeemable_ticket);
        r.method(Method::POST).with(tickets::redeem);
    }).resource("/users/me", |r| {
        r.method(Method::GET).with(users::current_user);
        r.method(Method::PUT).with(users::update_current_user);
    }).resource("/users/register", |r| {
        r.method(Method::POST).with(users::register)
    }).resource("/users", |r| {
        r.method(Method::GET).with(users::find_by_email);
    }).resource("/users/{id}", |r| {
        r.method(Method::GET).with(users::show);
    }).resource("/users/{id}/organizations", |r| {
        r.method(Method::GET).with(users::list_organizations);
    }).resource("/venues/{id}/events", |r| {
        r.method(Method::GET).with(events::show_from_venues);
    }).resource("/venues/{id}/organizations", |r| {
        r.method(Method::POST).with(venues::add_to_organization);
    }).resource("/venues/{id}/toggle_privacy", |r| {
        r.method(Method::PUT).with(venues::toggle_privacy);
    }).resource("/venues/{id}", |r| {
        r.method(Method::GET).with(venues::show);
        r.method(Method::PUT).with(venues::update);
    }).resource("/venues", |r| {
        r.method(Method::GET).with(venues::index);
        r.method(Method::POST).with(venues::create);
    }).register()
    .default_resource(|r| {
        r.method(Method::GET)
            .f(|_req| HttpResponse::NotFound().json(json!({"error": "Not found".to_string()})));
    })
}
