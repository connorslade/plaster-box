use std::collections::HashMap;
use std::fs;

use afire::{Logger, Method, Middleware, Query, Response, ServeStatic, Server};
use mut_static::MutStatic;
use uuid::Uuid;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref DATA: MutStatic<HashMap<Uuid, String>> = MutStatic::from(HashMap::new());
}

fn main() {
    let mut server = Server::new("localhost", 3030);

    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);

    server.route(Method::POST, "/new", |req| {
        let body_str = match req.body_string() {
            Some(i) => i,
            None => return Response::new().text("Invalid Text"),
        };

        let uuid = Uuid::new_v4();

        DATA.write().unwrap().insert(uuid, body_str);
        dbg!(&*DATA.read().unwrap());

        Response::new().text(uuid)
    });

    server.route(Method::GET, "/b/{id}", |req| {
        let id = req.path_param("id").unwrap();
        let data = DATA.read().unwrap();

        let data = match data.get(&Uuid::parse_str(&id).unwrap()) {
            Some(i) => i,
            None => return Response::new().status(404),
        };

        let template = fs::read_to_string("web/template/box.html")
            .unwrap()
            .replace("{{DATA}}", data)
            .replace("{{ID}}", &id);

        Response::new().text(template)
    });

    server.start().unwrap();
}
