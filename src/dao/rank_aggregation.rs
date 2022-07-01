use crate::models::{RankAggregation, RankAggregationCommand};
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{insert_into, pg::Pg, Connection, ExpressionMethods, QueryDsl, RunQueryDsl};

pub fn insert<T>(conn: &T, ra: RankAggregationCommand) -> Result<i32, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(rank_aggregations::table)
        .values(ra)
        .returning(rank_aggregations::id)
        .get_result(conn)
        .context("failed to insert rank aggregation")
}

pub fn update<T>(conn: &T, id: i32, ra: RankAggregationCommand) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    diesel::update(rank_aggregations::table.filter(rank_aggregations::id.eq(id)))
        .set(ra)
        .execute(conn)
        .context("failed to update rank aggregation")
}

pub fn get_for_update<T>(conn: &T, id: i32) -> Result<RankAggregation, Error>
where
    T: Connection<Backend = Pg>,
{
    rank_aggregations::table
        .filter(rank_aggregations::id.eq(id))
        .for_update()
        .first(conn)
        .context("failed to get rank aggregation for update")
}
