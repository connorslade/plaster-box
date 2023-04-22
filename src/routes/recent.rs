use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use afire::{Content, Method, Response, Server};
use rusqlite::params;
use unidecode::unidecode;
use uuid::Uuid;

use crate::{
    common::{best_time, safe_html},
    App,
};

const RECENT_PAGE_ITEMS: usize = 25;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/recent", |app, req| {
        let mut pages = String::new();
        let mut out = String::new();
        let mut page = 0;

        if let Some(i) = req.query.get("page") {
            page = i.parse::<usize>().unwrap_or(0);
        }

        let db = app.database.lock();
        let page_count = db.query_row("SELECT COUNT(*) FROM bins WHERE hidden = 0", [], |row| row.get::<_, usize>(0)).unwrap();
        for i in 0..=(page_count / RECENT_PAGE_ITEMS) {
            pages.push_str(&format!(
                r#"<li><a class="pagination-link{}" aria-label="Goto page {i}" href="?page={i}">{i}</a></li>"#,
                if i == page { " is-current" } else { "" },
            ));
        }

        let mut bins = db.prepare(include_str!("../sql/query_recent.sql")).unwrap();
        let mut bins = bins.query(params![RECENT_PAGE_ITEMS, page * RECENT_PAGE_ITEMS]).unwrap();
        while let Some(i) = bins.next().unwrap() {
            let uuid = i.get::<_, String>(0).unwrap();
            let time = i.get::<_, u64>(2).unwrap();
            let mut name = safe_html(&unidecode(&i.get::<_, String>(1).unwrap()));
            if name.len() > 50 {
                name = name.chars().take(50).collect();
            }

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards!")
                .as_secs();

            out.push_str(&format!(
                r#"<tr id="{id}"><td>{name}</td><td>{id}</td><td>{date} ago</td></tr>"#,
                id = Uuid::parse_str(&uuid).unwrap(),
                date = best_time(current_time - time)
            ));
        }

        let template = fs::read_to_string("web/template/recent.html")
            .unwrap()
            .replace("{{PAGES}}", &pages)
            .replace("{{DATA}}", &out);

        Response::new().text(template).content(Content::HTML)
    });
}
