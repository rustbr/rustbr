extern crate iron;
extern crate router;
extern crate params;
extern crate persistent;
extern crate chrono;
extern crate time;

use libraries::db::Database;
use libraries::jwt::SessionToken;
use schemas;
use models::User;
use persistent::Read;
use iron::prelude::*;
use iron::status;
use params::{Params, Value};
use self::chrono::*;

pub fn route (router: &mut router::Router) {
    router.post("/v1/sessions", create);
}

fn create (req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<Read<Database>>().unwrap();
    let payload = req.get_ref::<Params>().unwrap();
    let mut response = Response::new();

    let email = match payload.get("email") {
        Some(&Value::String(ref email)) => email.clone(),
        _ => panic!()
    };

    let password = match payload.get("password") {
        Some(&Value::String(ref password)) => password.clone(),
        _ => panic!()
    };


    let db_time = time::precise_time_ns();
    let user = User::find_by_email(pool, email);
    println!("db time {:?}ns", time::precise_time_ns() - db_time);
    let bcrypt_time = time::precise_time_ns();
    let valid = user.check_password(password);
    println!("bcrypt time {:?}ns", time::precise_time_ns() - bcrypt_time);
    let token = SessionToken::new(user.id);

    response.set_mut(status::Ok);
    response.set_mut(schemas::Token::new(token));

    Ok(response)
}
