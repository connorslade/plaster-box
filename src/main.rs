use std::fs;
use std::time::Instant;

use afire::{Content, Logger, Method, Middleware, Response, ServeStatic, Server};
use mut_static::MutStatic;
use uuid::Uuid;
#[macro_use]
extern crate lazy_static;

const DATA_LIMIT: usize = 256_000;

const TIME_UNITS: &[(&str, u16)] = &[
    ("second", 60),
    ("minute", 60),
    ("hour", 24),
    ("day", 30),
    ("month", 12),
    ("year", 0),
];

#[derive(Debug)]
pub struct Bin {
    uuid: Uuid,
    data: String,
    name: String,
    time: Instant,
}

lazy_static! {
    pub static ref DATA: MutStatic<Vec<Bin>> = MutStatic::from(Vec::new());
}

fn main() {
    let mut server = Server::new("localhost", 3030);

    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);

    server.route(Method::POST, "/new", |req| {
        if req.body.len() > DATA_LIMIT {
            return Response::new().text("Data too big!");
        }

        let body_str = match req.body_string() {
            Some(i) => i,
            None => return Response::new().text("Invalid Text"),
        };

        let name = req
            .header("Name")
            .unwrap_or_else(|| "Untitled Box".to_owned());

        let uuid = Uuid::new_v4();
        let bin = Bin {
            uuid,
            name,
            data: body_str,
            time: Instant::now(),
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
            .replace("{{NAME}}", &data.name)
            .replace("{{ID}}", uuid.to_string().as_str());

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

        Response::new()
            .text(&data.data)
            .content(Content::Custom("text/plain; charset=UTF-8"))
    });

    server.route(Method::GET, "/recent", |_| {
        let data = DATA.read().unwrap();
        let mut out = String::new();

        for (i, item) in (&*data).iter().rev().take(50).enumerate() {
            if i > 15 {
                break;
            }

            let mut name = item.name.as_str();

            if name.len() > 50 {
                name = &name[..50];
            }

            out.push_str(&format!(
                r#"<tr id="{id}"><td>{name}</td><td>{id}</td><td>{date}</td></tr>"#,
                id = item.uuid,
                date = best_time(item.time.elapsed().as_secs())
            ));
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

pub fn best_time(secs: u64) -> String {
    let mut secs = secs as f64;

    for i in TIME_UNITS {
        if i.1 == 0 || secs < i.1 as f64 {
            secs = secs.round();
            return format!("{} {}{}", secs, i.0, if secs > 1.0 { "s" } else { "" });
        }

        secs /= i.1 as f64;
    }

    format!("{} years", secs.round())
}
