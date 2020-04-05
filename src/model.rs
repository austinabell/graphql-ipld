use juniper::Context;
use juniper::{FieldError, FieldResult};
use parking_lot::RwLock;
use std::collections::HashMap;

pub struct Database {
    humans: RwLock<HashMap<String, Human>>,
}

impl Context for Database {}

#[derive(juniper::GraphQLEnum, Clone)]
pub enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(juniper::GraphQLObject, Clone)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
pub struct NewHuman {
    pub name: String,
    pub appears_in: Vec<Episode>,
    pub home_planet: String,
}

impl Database {
    pub fn new() -> Database {
        let mut humans = HashMap::new();

        humans.insert(
            "1000".to_owned(),
            Human {
                id: "1000".to_owned(),
                name: "Luke Skywalker".to_owned(),
                appears_in: vec![Episode::NewHope, Episode::Empire, Episode::Jedi],
                home_planet: "Tatooine".to_owned(),
            },
        );

        Database {
            humans: RwLock::new(humans),
        }
    }
    pub fn insert_human(&self, human: NewHuman) -> Human {
        let human = Human {
            id: "1002".to_owned(),
            name: human.name,
            appears_in: human.appears_in,
            home_planet: human.home_planet,
        };
        self.humans.write().insert("1002".to_owned(), human.clone());
        human
    }
    pub fn find_human(&self, id: &str) -> FieldResult<Human> {
        self.humans.read().get(id).cloned().ok_or_else(|| {
            FieldError::new(
                "Could not open connection to the database",
                graphql_value!({ "internal_error": "Connection refused" }),
            )
        })
    }
}
