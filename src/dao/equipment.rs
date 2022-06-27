use crate::models::{Equipment, EquipmentCommand, Location};
use crate::schema::*;
use anyhow::{Context, Error};
use diesel::{insert_into, pg::Pg, BelongingToDsl, Connection, GroupedBy, QueryDsl, RunQueryDsl};

pub fn insert_equipment<T>(conn: &T, equip: EquipmentCommand) -> Result<i32, Error>
where
    T: Connection<Backend = Pg>,
{
    insert_into(equipments::table)
        .values(equip)
        .returning(equipments::id)
        .get_result(conn)
        .context("failed to insert equipment")
}

pub fn equipements_of_locations<T>(conn: &T, location: &Vec<Location>) -> Result<Vec<Vec<Equipment>>, Error>
where
    T: Connection<Backend = Pg>,
{
    Ok(Equipment::belonging_to(location).load(conn).context("failed to query equipments of locations")?.grouped_by(location))
}

pub fn equipements_of_location<T>(conn: &T, location: &Location) -> Result<Vec<Equipment>, Error>
where
    T: Connection<Backend = Pg>,
{
    Equipment::belonging_to(location).load(conn).context("failed to query equipments of locations")
}
