use std::process;
use std::time::Duration;

use afire::{
    extension::{Logger, ServeStatic},
    trace,
    trace::{set_log_level, Level},
    Middleware, Server,
};

mod app;
mod common;
mod config;
mod database;
mod routes;
use app::App;

use crate::database::Database;

fn main() {
    set_log_level(Level::Trace);
    let app = App::new();

    let threads = app.config.threads;
    let mut server = Server::<App>::new(app.config.host.as_str(), app.config.port)
        // Set server state
        .state(app)
        // Set default headers
        .default_header("X-Content-Type-Options", "nosniff")
        .default_header("X-Frame-Options", "DENY")
        // Set other things
        .default_header("X-Server", format!("afire/{}", afire::VERSION))
        .socket_timeout(Duration::from_secs(5));

    let end_app = server.state.as_ref().unwrap().clone();
    ctrlc::set_handler(move || {
        trace!("Saving database");
        end_app.db().cleanup().unwrap();
        process::exit(0);
    })
    .unwrap();

    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);
    routes::attach(&mut server);

    server.start_threaded(threads).unwrap();
}
