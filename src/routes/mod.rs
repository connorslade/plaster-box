use crate::{App, Server};

mod new;
mod recent;
mod root;
mod view;
mod view_raw;

pub fn attach(server: &mut Server<App>) {
    new::attach(server);
    recent::attach(server);
    root::attach(server);
    view::attach(server);
    view_raw::attach(server);
}
