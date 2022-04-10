use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use afire::{Content, Logger, Method, Middleware, Response, ServeStatic, Server};
use mut_static::MutStatic;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[macro_use]
extern crate lazy_static;

mod footer;

const DATA_LIMIT: usize = 256_000;
const SAVE_INTERVAL: u64 = 60 * 60;
const SAVE_FILE: &str = "data.db";
const RECENT_PAGE_ITEMS: usize = 25;

const TIME_UNITS: &[(&str, u16)] = &[
    ("second", 60),
    ("minute", 60),
    ("hour", 24),
    ("day", 30),
    ("month", 12),
    ("year", 0),
];

#[derive(Debug, Serialize, Deserialize)]
pub struct Bin {
    uuid: [u8; 16],
    data: String,
    name: String,
    time: u64,
}

lazy_static! {
    pub static ref DATA: MutStatic<Vec<Bin>> =
        MutStatic::from(Bin::load(PathBuf::from(SAVE_FILE)).unwrap());
}

fn main() {
    lazy_static::initialize(&DATA);

    let mut server = Server::new("localhost", 3030)
        // Set defult headers
        .default_header("X-Content-Type-Options", "nosniff")
        .default_header("X-Frame-Options", "DENY")
        // Set other things
        .default_header("X-Server", format!("afire/{}", afire::VERSION))
        .socket_timeout(Duration::from_secs(5));

    footer::Footer.attach(&mut server);
    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);

    thread::Builder::new()
        .name("Saver".to_string())
        .spawn(|| {
            thread::sleep(Duration::from_secs(SAVE_INTERVAL));
            println!("[*] Saveing");
            Bin::save(&*DATA.read().unwrap(), PathBuf::from(SAVE_FILE)).unwrap();
        })
        .unwrap();

    ctrlc::set_handler(|| {
        println!("[*] Saveing");
        Bin::save(&*DATA.read().unwrap(), PathBuf::from(SAVE_FILE)).unwrap();
        process::exit(0);
    })
    .unwrap();

    server.route(Method::GET, "/", |_| {
        Response::new()
            .status(308)
            .text(r#"<a href="/new">/new</a>"#)
            .header("Location", "/new")
            .content(Content::HTML)
    });

    server.route(Method::POST, "/new", |req| {
        if req.body.len() > DATA_LIMIT {
            return Response::new().status(400).text("Data too big!");
        }

        let body_str = match req.body_string() {
            Some(i) => i,
            None => return Response::new().status(400).text("Invalid Text"),
        };

        let name = req
            .header("Name")
            .unwrap_or_else(|| "Untitled Box".to_owned());

        let uuid = Uuid::new_v4();
        let bin = Bin {
            uuid: *uuid.as_bytes(),
            name,
            data: body_str,
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        DATA.write().unwrap().push(bin);

        Response::new().text(uuid)
    });

    server.route(Method::GET, "/b/{id}", |req| {
        let id = req.path_param("id").unwrap();
        let data = DATA.read().unwrap();

        let uuid = &Uuid::parse_str(&id).expect("Invalid UUID");
        let data = match data.iter().find(|x| x.uuid == *uuid.as_bytes()) {
            Some(i) => i,
            None => return Response::new().status(404).text("Bin not Found"),
        };

        let mut code_blocks = String::new();

        for i in safe_html(&data.data).lines() {
            code_blocks.push_str(&format!("<code>{}</code>", i));
        }

        let template = fs::read_to_string("web/template/box.html")
            .unwrap()
            .replace("{{DATA}}", &code_blocks)
            .replace("{{NAME}}", &safe_html(&data.name))
            .replace("{{ID}}", uuid.to_string().as_str());

        Response::new().text(template).content(Content::HTML)
    });

    server.route(Method::GET, "/raw/{id}", |req| {
        let id = req.path_param("id").unwrap();
        let data = DATA.read().unwrap();

        let uuid = &Uuid::parse_str(&id).expect("Invalid UUID");
        let data = match data.iter().find(|x| x.uuid == *uuid.as_bytes()) {
            Some(i) => i,
            None => return Response::new().status(404).text("Bin not Found"),
        };

        Response::new()
            .text(&data.data)
            .content(Content::Custom("text/plain; charset=UTF-8"))
    });

    server.route(Method::GET, "/recent", |req| {
        let data = DATA.read().unwrap();
        let mut pages = String::new();
        let mut out = String::new();
        let mut page = 0;

        if let Some(i) = req.query.get("page") {
            page = i.parse::<usize>().unwrap_or(0);
        }

        for (i, _item) in data.iter().step_by(RECENT_PAGE_ITEMS).enumerate() {
            pages.push_str(&format!(
                r#"<li><a class="pagination-link{}" aria-label="Goto page {i}" href="?page={i}">{i}</a></li>"#,
                if i == page { " is-current" } else { "" },
            ));
        }

        for item in (&*data)
            .iter()
            .rev()
            .skip(RECENT_PAGE_ITEMS * page)
            .take(RECENT_PAGE_ITEMS)
        {
            let mut name = safe_html(item.name.as_str());

            if name.len() > 50 {
                name = name[..50].to_owned();
            }

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards!")
                .as_secs();

            out.push_str(&format!(
                r#"<tr id="{id}"><td>{name}</td><td>{id}</td><td>{date} ago</td></tr>"#,
                id = Uuid::from_slice(&item.uuid).unwrap(),
                date = best_time(current_time - item.time)
            ));
        }

        let template = fs::read_to_string("web/template/recent.html")
            .unwrap()
            .replace("{{PAGES}}", &pages)
            .replace("{{DATA}}", &out);

        Response::new().text(template).content(Content::HTML)
    });

    server.start().unwrap();
}

impl Bin {
    fn save(inp: &[Self], file: PathBuf) -> Option<()> {
        let bin = bincode::serialize(&inp).ok()?;

        fs::write(file, bin).ok()?;

        Some(())
    }

    fn load(file: PathBuf) -> Option<Vec<Self>> {
        if !file.exists() {
            return Some(Vec::new());
        }

        let raw = fs::read(file).ok()?;
        let data: Vec<Self> = bincode::deserialize(&raw).ok()?;
        println!("[*] Loaded {} Item", data.len());

        Some(data)
    }
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
