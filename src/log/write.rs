use std::io::{self, stdout, IsTerminal, Write};
use super::{Logger, Level};

#[derive(Debug)]
pub struct ANSI{
    pub labels: bool,
    pub force_color : bool,
    pub colors: [(u8, u8, u8);3],
}

const GRY : u32 = 0x838383;
const YEL : u32 = 0xab8704;
const RED : u32 = 0xff2100;

impl ANSI {
    pub const fn new() -> Self{Self{
        labels: true,
        force_color: false,
        colors: [
            (
                ( ((GRY & 0xff0000) >> (8 * 2)) ) as u8,
                ( ((GRY & 0x00ff00) >> (8 * 1)) ) as u8,
                ( ((GRY & 0x0000ff) >> (8 * 0)) ) as u8,
            ),
            (
                ( ((YEL & 0xff0000) >> (8 * 2)) ) as u8,
                ( ((YEL & 0x00ff00) >> (8 * 1)) ) as u8,
                ( ((YEL & 0x0000ff) >> (8 * 0)) ) as u8,
            ),
            (
                ( ((RED & 0xff0000) >> (8 * 2)) ) as u8,
                ( ((RED & 0x00ff00) >> (8 * 1)) ) as u8,
                ( ((RED & 0x0000ff) >> (8 * 0)) ) as u8,
            ),
        ],
    }
    }
}

impl Logger for ANSI {
    fn log(&mut self, msg: String, level: super::Level) -> io::Result<()> {
        let mut out = stdout();
        use Level as L;
        let prefix = if !self.labels {
            String::new()
        }else{
            let (slvl, ilvl) = match level {
                L::Log     => ("Log ", 0),
                L::Warning => ("Warn", 1),
                L::Error   => ("Err ", 2),
            };
            if out.is_terminal() || self.force_color {
                let (r, g, b)= self.colors[ilvl];
                let color = format!("\x1b[38;2;{r};{g};{b}m");
                let reset = "\x1b[m";

                format!("[{color}{slvl}{reset}]:")
            } else{
                format!("[{slvl}]:")
            }
        };

        out.write_all(format!("{prefix}{msg}\n").as_bytes())
    }
}

impl<T> Logger for T where T : io::Write{
    fn log(&mut self, msg: String, _level: super::Level) -> io::Result<()> {
        self.write_all(msg.as_bytes())
    }
}
