use crate::model::{Database, Episode, Human, NewHuman};
use juniper::FieldResult;
use juniper::{graphql_interface, Context};

pub struct Query;

#[juniper::object(
    // Here we specify the context type for the object.
    // We need to do this in every type that
    // needs access to the context.
    Context = Database,
)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    // Arguments to resolvers can either be simple types or input objects.
    // To gain access to the context, we specify a argument
    // that is a reference to the Context type.
    // Juniper automatically injects the correct context here.
    fn human(context: &Database, id: String) -> FieldResult<Human> {
        // Execute a db query.
        let human = context.find_human(&id)?;
        // Return the result.
        Ok(human)
    }
}

// Now, we do the same for our Mutation type.

pub struct Mutation;

#[juniper::object(
    Context = Database,
)]
impl Mutation {
    fn createHuman(name: String, home_planet: String) -> FieldResult<Human> {
        let mut db = executor.context();
        let human: Human = db.insert_human(NewHuman {
            name,
            home_planet,
            appears_in: vec![],
        });
        Ok(human)
    }
}
