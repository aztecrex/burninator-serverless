extern crate juniper;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file! ("src/schema.graphql");

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;
impl QueryFields for Query {
    fn field_hello(&self, _executor: &juniper::Executor<'_, Context>, name: String) -> juniper::FieldResult<String> {
        Ok(format!("Hello, {}", name))
    }
    fn field_backlogs(
        &self,
        _executor: &juniper::Executor<'_, Context>,
        _trail: &QueryTrail<'_, Backlog, Walked>) ->
            juniper::FieldResult<Vec<Backlog>> {
        Ok(vec!(
            Backlog {id: "one".to_owned(), name: "The first backlog".to_owned()},
            Backlog {id: "2".to_owned(), name: "Another backlog".to_owned()},
            Backlog {id: "III".to_owned(), name: "Very important backlog".to_owned()},
            Backlog {id: "9-5".to_owned(), name: "Let's get rid of this one".to_owned()},
            Backlog {id: "SIX".to_owned(), name: "Brand new, who wants to work on me?".to_owned()},
        ))
    }
}
pub struct Backlog {
    id: String,
    name: String,
}

impl BacklogFields for Backlog {
    fn field_id(&self, _executor: &juniper::Executor<'_, Context>) -> juniper::FieldResult<juniper::ID> {
        Ok(juniper::ID::new(self.id.to_owned()))
    }
    fn field_name(&self, _executor: &juniper::Executor<'_, Context>) -> juniper::FieldResult<String> {
        Ok(self.name.to_owned())
    }
}

fn main() {
    let ctx = Context;

    let query = "query { helloWorld(name: \"Ferris\") }";

    let (result, errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, Mutation),
        &juniper::Variables::new(),
        &ctx,
    )
    .unwrap();

    assert_eq!(errors.len(), 0);
    assert_eq!(
        result
            .as_object_value()
            .unwrap()
            .get_field_value("helloWorld")
            .unwrap()
            .as_scalar_value::<String>()
            .unwrap(),
        "Hello, Ferris!",
    );
}
