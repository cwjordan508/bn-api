use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(PartialEq, Debug)]
pub enum Scopes {
    ArtistWrite,
    EventWrite,
    EventInterest,
    OrderMakeExternalPayment,
    OrgAdmin,
    OrgRead,
    OrgWrite,
    RegionWrite,
    UserRead,
    TicketAdmin,
    VenueWrite,
}

impl fmt::Display for Scopes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Scopes::ArtistWrite => "artist:write",
            Scopes::EventWrite => "event:write",
            Scopes::EventInterest => "event:interest",
            Scopes::OrderMakeExternalPayment => "order::make-external-payment",
            Scopes::OrgAdmin => "org:admin",
            Scopes::OrgRead => "org:read",
            Scopes::OrgWrite => "org:write",
            Scopes::RegionWrite => "region:write",
            Scopes::UserRead => "user:read",
            Scopes::VenueWrite => "venue:write",
            Scopes::TicketAdmin => "ticket:admin",
        };
        write!(f, "{}", s)
    }
}

pub fn get_scopes(roles: Vec<String>) -> Vec<String> {
    let scopes: Vec<Scopes> = roles.iter().flat_map(|r| get_scopes_for_role(r)).collect();
    let mut scopes: Vec<String> = scopes.iter().map(|s| s.to_string()).collect();
    scopes.sort();
    scopes.dedup();
    scopes
}

fn get_scopes_for_role(role: &str) -> Vec<Scopes> {
    match role {
        // More scopes will be available for users later
        "User" => {
            let mut roles = vec![Scopes::EventInterest];
            roles
        }
        "OrgMember" => {
            let mut roles = vec![
                Scopes::ArtistWrite,
                Scopes::EventWrite,
                Scopes::OrgRead,
                Scopes::TicketAdmin,
                Scopes::VenueWrite,
            ];
            roles.extend(get_scopes_for_role("User"));
            roles
        }
        "OrgOwner" => {
            let mut roles = vec![Scopes::OrgWrite, Scopes::UserRead];
            roles.extend(get_scopes_for_role("OrgMember"));
            roles
        }
        "Admin" => {
            let mut roles = vec![
                Scopes::OrderMakeExternalPayment,
                Scopes::OrgAdmin,
                Scopes::RegionWrite,
            ];
            roles.extend(get_scopes_for_role("OrgOwner"));
            roles
        }
        _ => Vec::<Scopes>::new(),
    }
}

#[test]
fn get_scopes_for_role_test() {
    let res = get_scopes_for_role("OrgOwner");
    assert_eq!(
        vec![
            Scopes::OrgWrite,
            Scopes::UserRead,
            Scopes::ArtistWrite,
            Scopes::EventWrite,
            Scopes::OrgRead,
            Scopes::TicketAdmin,
            Scopes::VenueWrite,
            Scopes::EventInterest,
        ],
        res
    );
}

#[test]
fn scopes_to_string() {
    assert_eq!("org:admin".to_string(), Scopes::OrgAdmin.to_string());
}

#[test]
fn get_scopes_test() {
    let mut res = get_scopes(vec!["OrgOwner".to_string()]);
    res.sort();
    assert_eq!(
        vec![
            "artist:write",
            "event:interest",
            "event:write",
            "org:read",
            "org:write",
            "ticket:admin",
            "user:read",
            "venue:write",
        ],
        res
    );
    let mut res = get_scopes(vec!["Admin".to_string()]);
    res.sort();
    assert_eq!(
        vec![
            "artist:write",
            "event:interest",
            "event:write",
            "order::make-external-payment",
            "org:admin",
            "org:read",
            "org:write",
            "region:write",
            "ticket:admin",
            "user:read",
            "venue:write",
        ],
        res
    );

    let res = get_scopes(vec!["OrgOwner".to_string(), "Admin".to_string()]);
    assert_eq!(
        vec![
            "artist:write",
            "event:interest",
            "event:write",
            "order::make-external-payment",
            "org:admin",
            "org:read",
            "org:write",
            "region:write",
            "ticket:admin",
            "user:read",
            "venue:write",
        ],
        res
    );
}
