use std::io;

use log::{set_logger, Level, Logger};

pub mod log;

struct StdoutLogger{ }

impl Logger for StdoutLogger {
    fn log(&mut self, msg: String, level: Level) -> io::Result<()>{
        println!("{}: {msg}", match level {
            Level::Log     => "\x1b[37m[LOG ]\x1b[0m",
            Level::Warning => "\x1b[33m[WARN]\x1b[0m",
            Level::Error   => "\x1b[1m\x1b[31m[ERR ]\x1b[0m",
        });

        Ok(())
    }
}

fn main() {
    let log = StdoutLogger {};
    set_logger(log);
    log!("test log  {}", 10);
    warn!("test warn {}", 10);
    error!("test err  {}", 10);
}
