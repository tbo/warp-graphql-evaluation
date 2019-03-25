#![deny(warnings)]

extern crate env_logger;
#[macro_use]
extern crate log as irrelevant_log;
extern crate juniper;
extern crate juniper_warp;
extern crate warp;

use juniper::*;
use warp::{http::Response, log, Filter};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field human(&executor, id: String) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

fn main() {
    pub fn schema() -> Schema {
        Schema::new(QueryRoot {}, MutationRoot {})
    }

    ::std::env::set_var("RUST_LOG", "warp_server");
    env_logger::init();

    let log = log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    info!("Listening on 127.0.0.1:3030");

    let state = warp::any().map(move || ());
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    warp::serve(
        warp::get2()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql"))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 3030));
}
