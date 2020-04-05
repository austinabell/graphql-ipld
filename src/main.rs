#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate juniper;

mod model;
mod schema;

#[cfg(all(test))]
mod tests;

use juniper::RootNode;
use model::IpldStore;
use rocket::response::content;
use rocket::State;
use schema::{Mutation, Query};

type Schema = RootNode<'static, Query, Mutation>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<IpldStore>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<IpldStore>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    rocket::ignite()
        .manage(IpldStore::default())
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
