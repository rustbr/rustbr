#[macro_use]
extern crate timeit;
extern crate iron;
extern crate logger;
extern crate persistent;
extern crate router;
extern crate params;
extern crate includedir;
extern crate phf;

pub mod libraries;
pub mod resources;
pub mod middlewares;
pub mod models;
pub mod schemas;

use self::libraries::db;
use iron::prelude::{Chain, Iron};
use router::Router;
use logger::Logger;

include!(concat!(env!("OUT_DIR"), "/assets.rs"));

fn main() {
    // Create middlewares
    let connection = db::connect().expect("Não foi possível conectar ao banco de dados");
    let mut router = Router::new();
    let (logger_before, logger_after) = Logger::new(None);

    // Register every resource
    resources::user::route(&mut router);
    resources::assets::route(&mut router);
    resources::sessions::route(&mut router);

    // Create the middleware chain
    let mut chain = Chain::new(router);

    // Setup database
    db::attach(connection, &mut chain);

    // Link everything
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:8080").unwrap();
}
