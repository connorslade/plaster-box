use std::fs;

use afire::{Content, Logger, Method, Middleware, Response, ServeStatic, Server};
use chrono::prelude::*;
use mut_static::MutStatic;
use uuid::Uuid;
#[macro_use]
extern crate lazy_static;

const DATA_LIMIT: usize = 256_000;

#[derive(Debug)]
pub struct Bin {
    uuid: Uuid,
    data: String,
    name: String,
    time: DateTime<Utc>,
}

lazy_static! {
    pub static ref DATA: MutStatic<Vec<Bin>> = MutStatic::from(Vec::new());
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

        if body_str.len() > DATA_LIMIT {
            return Response::new().text("Data too big!");
        }

        let name = req
            .header("Name")
            .unwrap_or_else(|| "Untitled Bin".to_owned());

        let uuid = Uuid::new_v4();
        let bin = Bin {
            uuid,
            name,
            data: body_str,
            time: Utc::now(),
        };

        dbg!(&bin);

        DATA.write().unwrap().push(bin);
        dbg!(&*DATA.read().unwrap());

        Response::new().text(uuid)
    });

    server.route(Method::GET, "/b/{id}", |req| {
        let id = req.path_param("id").unwrap();
        let data = DATA.read().unwrap();

        let uuid = &Uuid::parse_str(&id).unwrap();
        let data = match data.iter().find(|x| x.uuid == *uuid) {
            Some(i) => i,
            None => return Response::new().status(404),
        };

        let template = fs::read_to_string("web/template/box.html")
            .unwrap()
            .replace("{{DATA}}", &safe_html(&data.data))
            .replace("{{ID}}", &id);

        Response::new().text(template)
    });

    server.route(Method::GET, "/raw/{id}", |req| {
        let id = req.path_param("id").unwrap();
        let data = DATA.read().unwrap();

        let uuid = &Uuid::parse_str(&id).unwrap();
        let data = match data.iter().find(|x| x.uuid == *uuid) {
            Some(i) => i,
            None => return Response::new().status(404),
        };

        Response::new().text(&data.data).content(Content::TXT)
    });

    server.route(Method::GET, "/recent", |_| {
        let data = DATA.read().unwrap();
        let mut out = String::new();

        for (i, item) in (&*data).iter().enumerate() {
            if i > 15 {
                break;
            }

            out.push_str(&item.uuid.to_string());
            out.push_str("<br>")
        }

        let template = fs::read_to_string("web/template/recent.html")
            .unwrap()
            .replace("{{DATA}}", &out);

        Response::new().text(template).content(Content::HTML)
    });

    server.start().unwrap();
}

fn safe_html(html: &str) -> String {
    html.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
