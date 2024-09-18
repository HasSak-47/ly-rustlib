use std::io;
use log::{set_logger, write::ANSI};

pub mod log;

fn main() -> io::Result<()> {
    let log = ANSI::new();
    set_logger(log);
    log!("test log  {}", 10)?;
    warn!("test warn {}", 10)?;
    error!("test err  {}", 10)?;
    Ok(())
}
