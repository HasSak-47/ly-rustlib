pub mod write;
pub mod prelude{
    pub use crate::log::*;
    pub use crate::log::write::*;
    pub use crate::log::macros::*;
}

use std::{cmp, io, sync::{Arc, Mutex, OnceLock}};

#[derive(Default, PartialEq, Eq, PartialOrd)]
pub enum Level{
    #[default]
    Log,
    Warning,
    Error,
}

impl cmp::Ord for Level {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self == other{
            return cmp::Ordering::Equal;
        }
        use Level as L;
        match (self, other) {
            (L::Log, _) => cmp::Ordering::Less,
            (L::Error, _) => cmp::Ordering::Greater,
            (L::Warning, L::Log) => cmp::Ordering::Greater,
            (L::Warning, L::Error) => cmp::Ordering::Less,
            _ => unreachable!(),
        }
    }
}

pub trait Logger {
    fn write(&mut self, msg: String, level: Level) -> io::Result<()>;
    fn clear(&mut self) -> io::Result<()> { Ok(()) }

    fn log(&mut self, msg: String) -> io::Result<()>{
        self.write(msg, Level::Log)
    }

    fn error(&mut self, msg: String) -> io::Result<()>{
        self.write(msg, Level::Error)
    }

    fn warn(&mut self, msg: String) -> io::Result<()>{
        self.write(msg, Level::Warning)
    }
}

// temporal name
#[derive(Default)]
struct Out{
    logger: Option<Box<dyn Logger>>,
    level: Level,
}

unsafe impl Send for Out{ }

static LOGGER : OnceLock<Arc<Mutex<Out>>> = OnceLock::new();

pub fn set_logger<T: Logger + 'static>(new_logger : T){
    let arc = LOGGER.get_or_init( || Arc::new(Mutex::new(Out::default())) );
    arc.clone().lock().unwrap().logger.get_or_insert(Box::new(new_logger));
}

pub fn set_level(level: Level){
    let arc = LOGGER.get_or_init( || Arc::new(Mutex::new(Out::default())) );
    arc.clone().lock().unwrap().level = level;
}

pub fn write(msg: String, level: Level) -> io::Result<()>{
    let arc = LOGGER.get_or_init( || Arc::new(Mutex::new(Out::default())) );
    let binding = arc.clone();
    let out = &mut binding.lock().unwrap();
    if level < out.level{
        return Ok(());
    }

    match &mut out.logger{
        Some(s) => s.write(msg, level) ,
        _ => Ok(()),
    }
}

// damn rust, this kinda sucks
pub mod macros{
    #[macro_export]
    macro_rules! macro_log {
        ($fmt: tt $(, $params: expr)*) => {
            log_f(format!($fmt $(,$params)*))
        };
    }
    
    #[macro_export]
    macro_rules! macro_error {
        ($fmt: tt $(, $params: expr)*) => {
            error_f(format!($fmt $(,$params)*))
        };
    }

    #[macro_export]
    macro_rules! macro_warn {
        ($fmt: tt $(, $params: expr)*) => {
            warn_f(format!($fmt $(,$params)*))
        };
    }

    pub use crate::macro_warn as warn;
    pub use crate::macro_error as error;
    pub use crate::macro_log as log;
}

pub fn   log_f(msg: String) -> io::Result<()> { write(msg, Level::Log) }
pub fn  warn_f(msg: String) -> io::Result<()> { write(msg, Level::Warning) }
pub fn error_f(msg: String) -> io::Result<()> { write(msg, Level::Error) }


