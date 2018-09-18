use chrono::NaiveDateTime;
use diesel;
use diesel::expression::dsl;
use diesel::prelude::*;
use models::{Organization, Region};
use schema::{organization_users, venues};
use utils::errors::ConvertToDatabaseError;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use uuid::Uuid;

#[derive(
    Clone,
    Associations,
    Identifiable,
    Queryable,
    AsChangeset,
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
)]
#[belongs_to(Region)]
#[table_name = "venues"]
pub struct Venue {
    pub id: Uuid,
    pub region_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub is_private: bool,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset, Default, Deserialize)]
#[table_name = "venues"]
pub struct VenueEditableAttributes {
    pub region_id: Option<Uuid>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
}

#[derive(Default, Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "venues"]
pub struct NewVenue {
    pub name: String,
    pub region_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
}

impl NewVenue {
    pub fn commit(&self, connection: &PgConnection) -> Result<Venue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::InsertError,
            "Could not create new venue",
            diesel::insert_into(venues::table)
                .values(self)
                .get_result(connection),
        )
    }
}

impl Venue {
    pub fn create(name: &str, region_id: Option<Uuid>, organization_id: Option<Uuid>) -> NewVenue {
        NewVenue {
            name: String::from(name),
            region_id,
            organization_id,
            ..Default::default()
        }
    }

    pub fn update(
        &self,
        attributes: VenueEditableAttributes,
        conn: &PgConnection,
    ) -> Result<Venue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::UpdateError,
            "Could not update venue",
            diesel::update(self)
                .set((attributes, venues::updated_at.eq(dsl::now)))
                .get_result(conn),
        )
    }

    pub fn find(id: Uuid, conn: &PgConnection) -> Result<Venue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::QueryError,
            "Error loading venue",
            venues::table.find(id).first::<Venue>(conn),
        )
    }

    pub fn all(user_id: Option<Uuid>, conn: &PgConnection) -> Result<Vec<Venue>, DatabaseError> {
        let query = match user_id {
            Some(u) => venues::table
                .left_join(
                    organization_users::table.on(venues::organization_id
                        .eq(organization_users::organization_id.nullable())
                        .and(organization_users::user_id.eq(u))),
                ).filter(
                    organization_users::user_id
                        .eq(u)
                        .or(venues::is_private.eq(false)),
                ).order_by(venues::name)
                .select(venues::all_columns)
                .load(conn),
            None => venues::table
                .filter(venues::is_private.eq(false))
                .order_by(venues::name)
                .select(venues::all_columns)
                .load(conn),
        };

        query.to_db_error(ErrorCode::QueryError, "Unable to load all venues")
    }

    pub fn find_for_organization(
        user_id: Option<Uuid>,
        organization_id: Uuid,
        conn: &PgConnection,
    ) -> Result<Vec<Venue>, DatabaseError> {
        let query = match user_id {
            Some(u) => venues::table
                .left_join(
                    organization_users::table.on(venues::organization_id
                        .eq(organization_users::organization_id.nullable())
                        .and(organization_users::user_id.eq(u))),
                ).filter(
                    organization_users::user_id
                        .eq(u)
                        .or(venues::is_private.eq(false)),
                ).filter(venues::organization_id.eq(organization_id))
                .order_by(venues::name)
                .select(venues::all_columns)
                .load(conn),
            None => venues::table
                .filter(venues::is_private.eq(false))
                .filter(venues::organization_id.eq(organization_id))
                .order_by(venues::name)
                .select(venues::all_columns)
                .load(conn),
        };

        query.to_db_error(ErrorCode::QueryError, "Unable to load all venues")
    }

    pub fn add_to_organization(
        self,
        organization_id: &Uuid,
        conn: &PgConnection,
    ) -> Result<Venue, DatabaseError> {
        //Should I make sure that this venue doesn't already have one here even though there is logic
        //for that in the bn-api layer?
        diesel::update(&self)
            .set(venues::organization_id.eq(organization_id))
            .get_result(conn)
            .to_db_error(ErrorCode::UpdateError, "Could not update venue")
    }

    pub fn set_privacy(
        &self,
        is_private: bool,
        conn: &PgConnection,
    ) -> Result<Venue, DatabaseError> {
        DatabaseError::wrap(
            ErrorCode::UpdateError,
            "Could not update is_private for artist",
            diesel::update(self)
                .set((
                    venues::is_private.eq(is_private),
                    venues::updated_at.eq(dsl::now),
                )).get_result(conn),
        )
    }

    pub fn organization(&self, conn: &PgConnection) -> Result<Option<Organization>, DatabaseError> {
        match self.organization_id {
            Some(organization_id) => Ok(Some(Organization::find(organization_id, conn)?)),
            None => Ok(None),
        }
    }
}
