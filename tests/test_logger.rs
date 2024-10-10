use ly::log::*;

#[test]
fn test_ansi_log() -> std::io::Result<()> {
    use std::io;
    use ly::log::prelude::*;
    let log = ANSI::new();
    set_logger(log);
    log!("test log  {}", 10)?;
    warn!("test warn {}", 10)?;
    error!("test err  {}", 10)?;
    Ok(())
}
