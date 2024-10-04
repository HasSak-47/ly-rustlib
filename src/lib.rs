pub mod log;
pub mod proc{
    pub use util_macros::*;
}

mod main_test{
    #[test]
    fn test_ansi_log() -> std::io::Result<()> {
        use std::io;
        use crate::log::prelude::*;
        let log = ANSI::new();
        set_logger(log);
        log!("test log  {}", 10)?;
        warn!("test warn {}", 10)?;
        error!("test err  {}", 10)?;
        Ok(())
    }
}
