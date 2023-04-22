use std::fs;

use afire::{Content, Method, Response, Server};
use uuid::Uuid;

use crate::{common::safe_html, database::Database, App};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/b/{id}", |app, req| {
        let id = req.param("id").unwrap();

        let uuid = match Uuid::parse_str(&id) {
            Ok(i) => i,
            Err(_) => return Response::new().status(400).text("Invalid UUID"),
        };

        let bin = match app.db().get_bin(uuid) {
            Ok(Some(i)) => i,
            Ok(None) => {
                return Response::new()
                    .status(404)
                    .text("Bin not Found")
                    .content(Content::TXT)
            }
            Err(e) => panic!("{e}"),
        };

        let mut code_blocks = String::new();
        for i in safe_html(&bin.body).lines() {
            code_blocks.push_str(&format!("<code>{i}</code>"));
        }

        let template = fs::read_to_string("web/template/box.html")
            .unwrap()
            .replacen("{{DATA}}", &code_blocks, 1)
            .replacen("{{NAME}}", &safe_html(&bin.name), 1)
            .replacen("{{ID}}", uuid.to_string().as_str(), 2);

        Response::new().text(template).content(Content::HTML)
    });
}
