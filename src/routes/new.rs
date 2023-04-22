use afire::{internal::encoding::decode_url, Content, Method, Response, Server};
use rusqlite::params;
use uuid::Uuid;

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/new", |app, req| {
        if req.body.len() > app.config.data_limit {
            return Response::new().status(400).text("Data too big!");
        }

        let body_str = String::from_utf8_lossy(&req.body);
        let name = match req.headers.get("Name") {
            Some(i) => decode_url(i).unwrap(),
            None => "Untitled Box".to_owned(),
        };
        let hidden = req.headers.get("Hidden") == Some("true");
        let uuid = Uuid::new_v4();

        app.database
            .lock()
            .execute(
                include_str!("../sql/insert_bin.sql"),
                params![uuid.to_string(), body_str, name, hidden as u8],
            )
            .unwrap();

        Response::new().text(uuid).content(Content::TXT)
    });
}
