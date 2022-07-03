use crate::models::{Location, RankAggregation, RankAggregationInsert, RankAggregationUpdate};
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{insert_into, pg::Pg, BelongingToDsl, Connection, ExpressionMethods, GroupedBy, QueryDsl, RunQueryDsl};

pub fn insert<T>(conn: &T, ra: RankAggregationInsert) -> Result<i32, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(rank_aggregations::table)
        .values(ra)
        .returning(rank_aggregations::id)
        .get_result(conn)
        .context("failed to insert rank aggregation")
}

pub fn update<T>(conn: &T, loc: i32, ra: RankAggregationUpdate) -> Result<usize, Error>
where
    T: Connection<Backend = Pg>,
{
    diesel::update(rank_aggregations::table.filter(rank_aggregations::location_id.eq(loc)))
        .set(ra)
        .execute(conn)
        .context("failed to update rank aggregation")
}

pub fn get_for_update<T>(conn: &T, loc: i32) -> Result<RankAggregation, Error>
where
    T: Connection<Backend = Pg>,
{
    rank_aggregations::table
        .filter(rank_aggregations::location_id.eq(loc))
        .for_update()
        .first(conn)
        .context("failed to get rank aggregation for update")
}

pub fn rank_aggs_of_location<T>(conn: &T, locs: &Vec<Location>) -> Result<Vec<RankAggregation>, Error>
where
    T: Connection<Backend = Pg>,
{
    let l: Vec<Vec<RankAggregation>> = RankAggregation::belonging_to(locs).load::<RankAggregation>(conn)?.grouped_by(locs);
    Ok(l.into_iter().flatten().collect())
}
