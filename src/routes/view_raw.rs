use afire::{Content, Method, Response, Server};
use rusqlite::Error;
use uuid::Uuid;

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/raw/{id}", |app, req| {
        let id = req.param("id").unwrap();

        let uuid = match Uuid::parse_str(&id) {
            Ok(i) => i,
            Err(_) => return Response::new().status(400).text("Invalid UUID"),
        };

        let data = match app.database.lock().query_row(
            "SELECT data FROM bins WHERE uuid = ?",
            [uuid.to_string()],
            |row| row.get::<_, String>(0),
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

        Response::new()
            .text(data)
            .header("Access-Control-Allow-Origin", "*")
            .content(Content::TXT)
    });
}
