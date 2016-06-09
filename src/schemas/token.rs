extern crate serde;
extern crate serde_json;
extern crate iron;
extern crate chrono;
extern crate time;

use self::iron::modifier::Modifier;
use self::iron::prelude::*;
use self::serde_json as json;
use self::iron::headers;
use std::collections::BTreeMap;
use libraries::jwt;
use self::chrono::*;

pub struct Token {
    map: BTreeMap<String, String>
}

impl Token {
    pub fn new(token: jwt::SessionToken) -> Token {
        let mut map = BTreeMap::new();
        let jwt_time = time::precise_time_ns();
        map.insert("token".to_string(), token.to_string());
        println!("jwt time {:?}ns", time::precise_time_ns() - jwt_time);
        Token {
            map: map
        }
    }
}

impl Modifier<Response> for Token {
    fn modify(self, resp: &mut Response) {
        resp.headers.set(headers::ContentType::json());
        let srl_time = time::precise_time_ns();
        resp.set_mut(json::to_string(&self.map).unwrap());
        println!("json time {:?}ns", time::precise_time_ns() - srl_time);
    }
}
