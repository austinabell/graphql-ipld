use crate::model::{GQLIpld, IpldStore};
use juniper::FieldResult;

pub struct Query;

#[juniper::object(
    Context = IpldStore,
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn resolve(context: &IpldStore, cid: String) -> FieldResult<GQLIpld> {
        // Execute a db query.
        let data = context.retrieve_ipld(&cid.parse()?)?;
        // Return the result.
        Ok(data)
    }
}

pub struct Mutation;

#[juniper::object(
    Context = IpldStore,
)]
impl Mutation {
    fn insertIpld(value: i32) -> FieldResult<String> {
        let mut db = executor.context();
        let cid = db.insert_ipld(value)?;
        Ok(cid.to_string())
    }
}
