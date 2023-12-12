#[allow(dead_code)]
pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Reset,
}

impl std::fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Colors::Black => "\x1b[30m",
            Colors::Red => "\x1b[31m",
            Colors::Green => "\x1b[32m",
            Colors::Yellow => "\x1b[33m",
            Colors::Blue => "\x1b[34m",
            Colors::Magenta => "\x1b[35m",
            Colors::Cyan => "\x1b[36m",
            Colors::LightGray => "\x1b[37m",
            Colors::DarkGray => "\x1b[90m",
            Colors::LightRed => "\x1b[91m",
            Colors::LightGreen => "\x1b[92m",
            Colors::LightYellow => "\x1b[93m",
            Colors::LightBlue => "\x1b[94m",
            Colors::LightMagenta => "\x1b[95m",
            Colors::LightCyan => "\x1b[96m",
            Colors::White => "\x1b[97m",
            Colors::Reset => "\x1b[0m",
        };
        f.write_str(val)
    }
}
