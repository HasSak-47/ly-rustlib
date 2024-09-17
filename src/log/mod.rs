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
    fn log(&mut self, msg: String, level: Level) -> io::Result<()>;
    fn clear(&mut self) -> io::Result<()> { Ok(()) }
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

pub fn write(msg: String, level: Level){
    let arc = LOGGER.get_or_init( || Arc::new(Mutex::new(Out::default())) );
    let binding = arc.clone();
    let out = &mut binding.lock().unwrap();
    if level < out.level{
        return;
    }

    match &mut out.logger{
        Some(s) => s.log(msg, level) ,
        _ => {},
    }
}

pub fn log(msg: String){ write(msg, Level::Log); }
pub fn warn(msg: String){ write(msg, Level::Warning); }
pub fn error(msg: String){ write(msg, Level::Error); }

#[macro_export]
macro_rules! log {
    ($fmt: tt $(, $params: expr)*) => {
        log::log(format!($fmt $(,$params)*));
    };
}

#[macro_export]
macro_rules! error {
    ($fmt: tt $(, $params: expr)*) => {
        log::error (format!($fmt $(,$params)*));
    };
}
#[macro_export]
macro_rules! warn {
    ($fmt: tt $(, $params: expr)*) => {
        log::warn(format!($fmt $(,$params)*));
    };
}
