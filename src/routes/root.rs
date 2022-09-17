use afire::{Content, Method, Response, Server};

use crate::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/", |_| {
        Response::new()
            .status(308)
            .text(r#"<a href="/new">/new</a>"#)
            .header("Location", "/new")
            .content(Content::HTML)
    });
}
