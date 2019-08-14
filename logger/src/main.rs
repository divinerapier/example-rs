#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::Env;

fn main() {
    env_logger::from_env(Env::default().default_filter_or("trace")).init();
    trace!("start up");
    debug!("start up");
    info!("start up");
    warn!("start up");
    error!("start up");
}
