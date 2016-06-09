extern crate iron;
extern crate router;
extern crate params;
extern crate persistent;

use libraries::db::Database;
use models::User;
use persistent::Read;
use iron::prelude::*;
use iron::status;
use params::{Params, Value};

pub fn route (router: &mut router::Router) {
    router.post("/v1/users", create);
}

fn create (req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<Database>>().unwrap();
    let payload = req.get_ref::<Params>().unwrap();

    let email = match payload.get("email") {
        Some(&Value::String(ref email)) => email.clone(),
        _ => panic!()
    };

    let first_name = match payload.get("first_name") {
        Some(&Value::String(ref first_name)) => first_name.clone(),
        _ => panic!()
    };

    let last_name = match payload.get("last_name") {
        Some(&Value::String(ref last_name)) => last_name.clone(),
        _ => panic!()
    };

    let password = match payload.get("password") {
        Some(&Value::String(ref password)) => password.clone(),
        _ => panic!()
    };

    let user = User::new(
        pool,
        email,
        first_name,
        last_name,
        password
    );

    println!("User created {:?}", user.id);

    Ok(Response::with((status::Ok)))
}
