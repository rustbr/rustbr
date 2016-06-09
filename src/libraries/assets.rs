use FILES;
use std::io;
use std::vec;
use iron::modifier::Modifier;
use iron::prelude::*;
use std::path::Path;
use std::result;

pub struct Asset {
    pub path: String,
    pub size: usize,
    pub data: vec::Vec<u8>
}

impl Modifier<Response> for Asset {
    fn modify(self, resp: &mut Response) {
        resp.set_mut(self.data);
        resp.set_mut(Path::new(&self.path));
    }
}

pub fn get (path: &str) -> result::Result<Asset, io::Error> {
    match FILES.get(path) {
        Ok(data) => {
            let bytes = data.into_owned();

            Ok(Asset {
                path: path.to_string(),
                size: bytes.len(),
                data: bytes
            })
        },
        Err(err) => Err(err)
    }
}
