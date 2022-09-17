use afire::{Content, Method, Response, Server};
use rusqlite::params;
use urlencoding::decode;
use uuid::Uuid;

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/new", |app, req| {
        if req.body.len() > app.config.data_limit {
            return Response::new().status(400).text("Data too big!");
        }

        let body_str = match req.body_string() {
            Some(i) => i,
            None => return Response::new().status(400).text("Invalid Text"),
        };

        let name = match req.header("Name") {
            Some(i) => decode(&i).unwrap().to_string(),
            None => "Untitled Box".to_owned(),
        };
        let uuid = Uuid::new_v4();

        app.database
            .lock()
            .execute(
                include_str!("../sql/insert_bin.sql"),
                params![uuid.to_string(), body_str, name],
            )
            .unwrap();

        Response::new().text(uuid).content(Content::TXT)
    });
}
