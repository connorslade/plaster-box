use std::fs;

use afire::{Content, Method, Response, Server};
use rusqlite::Error;
use uuid::Uuid;

use crate::{common::safe_html, App};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/b/{id}", |app, req| {
        let id = req.param("id").unwrap();

        let uuid = match Uuid::parse_str(&id) {
            Ok(i) => i,
            Err(_) => return Response::new().status(400).text("Invalid UUID"),
        };

        let (data, name) = match app.database.lock().query_row(
            "SELECT data, name FROM bins WHERE uuid = ?",
            [uuid.to_string()],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        ) {
            Ok(i) => i,
            Err(Error::QueryReturnedNoRows) => {
                return Response::new()
                    .status(404)
                    .text("Bin not Found")
                    .content(Content::TXT)
            }
            Err(e) => panic!("{}", e),
        };

        let mut code_blocks = String::new();
        for i in safe_html(&data).lines() {
            code_blocks.push_str(&format!("<code>{i}</code>"));
        }

        let template = fs::read_to_string("web/template/box.html")
            .unwrap()
            .replace("{{DATA}}", &code_blocks)
            .replace("{{NAME}}", &safe_html(&name))
            .replace("{{ID}}", uuid.to_string().as_str());

        Response::new().text(template).content(Content::HTML)
    });
}
