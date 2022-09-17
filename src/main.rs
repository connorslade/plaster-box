use std::process;
use std::time::Duration;

use afire::{
    extension::{Logger, ServeStatic},
    Middleware, Server,
};

mod app;
mod common;
mod config;
mod routes;
use app::App;

fn main() {
    let app = App::new();

    let mut server = Server::<App>::new(&app.config.host, app.config.port)
        // Set server state
        .state(app)
        // Set defult headers
        .default_header("X-Content-Type-Options", "nosniff")
        .default_header("X-Frame-Options", "DENY")
        // Set other things
        .default_header("X-Server", format!("afire/{}", afire::VERSION))
        .socket_timeout(Duration::from_secs(5));

    let error_app = server.state.as_ref().unwrap().clone();
    ctrlc::set_handler(move || {
        error_app
            .database
            .lock()
            .pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();
        process::exit(0);
    })
    .unwrap();

    ServeStatic::new("web/static").attach(&mut server);
    Logger::new().attach(&mut server);
    routes::attach(&mut server);

    server.start().unwrap();
}
