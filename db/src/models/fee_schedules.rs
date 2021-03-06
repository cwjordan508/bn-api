use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use models::{FeeScheduleRange, NewFeeScheduleRange};
use schema::{fee_schedule_ranges, fee_schedules};
use utils::errors::ConvertToDatabaseError;
use utils::errors::DatabaseError;
use utils::errors::ErrorCode;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Clone, Debug)]
pub struct FeeSchedule {
    pub id: Uuid,
    pub name: String,
    pub version: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl FeeSchedule {
    pub fn create(name: String, ranges: Vec<(NewFeeScheduleRange)>) -> NewFeeSchedule {
        NewFeeSchedule { name, ranges }
    }

    pub fn ranges(&self, conn: &PgConnection) -> Result<Vec<FeeScheduleRange>, DatabaseError> {
        fee_schedule_ranges::table
            .filter(fee_schedule_ranges::fee_schedule_id.eq(self.id))
            .load(conn)
            .to_db_error(ErrorCode::QueryError, "Could not load fee schedule ranges")
    }

    pub fn find(id: Uuid, conn: &PgConnection) -> Result<FeeSchedule, DatabaseError> {
        fee_schedules::table
            .find(id)
            .first::<FeeSchedule>(conn)
            .to_db_error(ErrorCode::QueryError, "Error loading Fee Schedule")
    }

    pub fn get_range(
        &self,
        price: i64,
        conn: &PgConnection,
    ) -> Result<Option<FeeScheduleRange>, DatabaseError> {
        let ranges: Vec<FeeScheduleRange> = fee_schedule_ranges::table
            .filter(fee_schedule_ranges::fee_schedule_id.eq(self.id))
            .order_by(fee_schedule_ranges::min_price.asc())
            .load(conn)
            .to_db_error(ErrorCode::QueryError, "Could not load fee schedule ranges")?;

        let mut found_range = None;

        for r in 0..ranges.len() {
            if ranges[r].min_price > price {
                break;
            }
            found_range = Some(ranges[r].clone());
        }

        Ok(found_range)
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewFeeSchedule {
    pub name: String,
    pub ranges: Vec<NewFeeScheduleRange>,
}

impl NewFeeSchedule {
    pub fn commit(self, conn: &PgConnection) -> Result<FeeSchedule, DatabaseError> {
        let previous_version = fee_schedules::table
            .filter(fee_schedules::name.eq(&self.name))
            .order_by(fee_schedules::id.desc())
            .first::<FeeSchedule>(conn)
            .optional()
            .to_db_error(ErrorCode::QueryError, "Error loading Fee Schedule")?;

        let next_version = match previous_version {
            None => 0,

            Some(pv) => pv.version + 1,
        };

        let result: FeeSchedule = diesel::insert_into(fee_schedules::table)
            .values((
                fee_schedules::name.eq(&self.name),
                fee_schedules::version.eq(next_version),
            )).get_result(conn)
            .to_db_error(ErrorCode::InsertError, "Could not create fee schedule")?;

        #[derive(Insertable)]
        #[table_name = "fee_schedule_ranges"]
        struct I {
            fee_schedule_id: Uuid,
            min_price: i64,
            fee_in_cents: i64,
        }
        let mut ranges = Vec::<I>::new();
        for range in &self.ranges {
            ranges.push(I {
                fee_schedule_id: result.id,
                min_price: range.min_price,
                fee_in_cents: range.fee_in_cents,
            })
        }
        diesel::insert_into(fee_schedule_ranges::table)
            .values(ranges)
            .execute(conn)
            .to_db_error(
                ErrorCode::InsertError,
                "Could not create fee schedule range",
            )?;

        Ok(result)
    }
}
