pub mod write;
pub mod prelude{
    pub use super::*;
    pub use super::write::*;
    pub use super::macros::*;

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

// damn rust this kinda sucks
pub mod macros{
    #[macro_export]
    #[doc(hidden)]
    macro_rules! marco_log {
        ($fmt: tt $(, $params: expr)*) => {
            super::log::log(format!($fmt $(,$params)*))
        };
    }
    
    #[macro_export]
    #[doc(hidden)]
    macro_rules! marco_error {
        ($fmt: tt $(, $params: expr)*) => {
            super::log::error (format!($fmt $(,$params)*))
        };
    }

    #[macro_export]
    #[doc(hidden)]
    macro_rules! marco_warn {
        ($fmt: tt $(, $params: expr)*) => {
            super::log::warn(format!($fmt $(,$params)*))
        };
    }

    pub use crate::marco_warn as warn;
    pub use crate::marco_error as error;
    pub use crate::marco_log as log;
}

pub fn   log(msg: String) -> io::Result<()> { write(msg, Level::Log) }
pub fn  warn(msg: String) -> io::Result<()> { write(msg, Level::Warning) }
pub fn error(msg: String) -> io::Result<()> { write(msg, Level::Error) }


