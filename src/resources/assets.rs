extern crate iron;
extern crate router;
extern crate params;

use libraries::assets;
use iron::prelude::*;
use iron::status;
use router::Router;

const PATH: &'static str = "assets/";

pub fn route (router: &mut router::Router) {
    router.get("/assets/*path", get);
    router.get("/", index);
}

fn index (_req: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(assets::get("assets/index.html").unwrap());
    Ok(response)
}

fn get (req: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let path = req.extensions.get::<Router>()
    .unwrap()
    .find("path")
    .unwrap();

    let mut asset_path = String::new();
    asset_path.push_str(&PATH);
    asset_path.push_str(path);

    match assets::get(&asset_path) {
        Ok(asset) => {
            response.set_mut(status::Ok);
            response.set_mut(asset);
        },
        Err(_) => {
            response.set_mut(status::NotFound);
        }
    };

    Ok(response)
}
