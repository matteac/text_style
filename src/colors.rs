use std::fmt::Display;

#[allow(dead_code)]
enum Foreground {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGrey,
    Grey,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    Strong,
}

trait Stringed {
    fn stringed(&self) -> String;
}

impl Stringed for Foreground {
    fn stringed(&self) -> String {
        let clr = match self {
            Self::Black => "\x1b[30m",
            Self::Red => "\x1b[31m",
            Self::Green => "\x1b[32m",
            Self::Yellow => "\x1b[33m",
            Self::Blue => "\x1b[34m",
            Self::Magenta => "\x1b[35m",
            Self::Cyan => "\x1b[36m",
            Self::LightGrey => "\x1b[37m",
            Self::Grey => "\x1b[90m",
            Self::LightRed => "\x1b[91m",
            Self::LightGreen => "\x1b[92m",
            Self::LightYellow => "\x1b[93m",
            Self::LightBlue => "\x1b[94m",
            Self::LightMagenta => "\x1b[95m",
            Self::LightCyan => "\x1b[96m",
            Self::White => "\x1b[97m",
            Self::Strong => "\x1b[1m",
        };
        clr.to_string()
    }
}

impl Display for Foreground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringed())
    }
}

#[allow(dead_code)]
enum Background {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGrey,
    Grey,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
}

impl Stringed for Background {
    fn stringed(&self) -> String {
        let clr = match self {
            Self::Black => "\x1b[40m",
            Self::Red => "\x1b[41m",
            Self::Green => "\x1b[42m",
            Self::Yellow => "\x1b[43m",
            Self::Blue => "\x1b[44m",
            Self::Magenta => "\x1b[45m",
            Self::Cyan => "\x1b[46m",
            Self::LightGrey => "\x1b[47m",
            Self::Grey => "\x1b[100m",
            Self::LightRed => "\x1b[101m",
            Self::LightGreen => "\x1b[102m",
            Self::LightYellow => "\x1b[103m",
            Self::LightBlue => "\x1b[104m",
            Self::LightMagenta => "\x1b[105m",
            Self::LightCyan => "\x1b[106m",
            Self::White => "\x1b[107m",
        };
        clr.to_string()
    }
}

impl Display for Background {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringed())
    }
}

#[allow(dead_code)]
enum Style {
    None,
    Bold,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Crossed,
}

impl Stringed for Style {
    fn stringed(&self) -> String {
        let clr = match self {
            Self::None => "\x1b[0m",
            Self::Bold => "\x1b[1m",
            Self::Italic => "\x1b[3m",
            Self::Underline => "\x1b[4m",
            Self::Blink => "\x1b[5m",
            Self::Reverse => "\x1b[7m",
            Self::Hidden => "\x1b[8m",
            Self::Crossed => "\x1b[9m",
        };
        clr.to_string()
    }
}

impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringed())
    }
}

pub trait Colored {
    fn bg_black(self) -> String;
    fn bg_red(self) -> String;
    fn bg_green(self) -> String;
    fn bg_yellow(self) -> String;
    fn bg_blue(self) -> String;
    fn bg_magenta(self) -> String;
    fn bg_cyan(self) -> String;
    fn bg_light_grey(self) -> String;
    fn bg_grey(self) -> String;
    fn bg_light_red(self) -> String;
    fn bg_light_green(self) -> String;
    fn bg_light_yellow(self) -> String;
    fn bg_light_blue(self) -> String;
    fn bg_light_magenta(self) -> String;
    fn bg_light_cyan(self) -> String;
    fn bg_white(self) -> String;

    fn fg_black(self) -> String;
    fn red(self) -> String;
    fn green(self) -> String;
    fn yellow(self) -> String;
    fn blue(self) -> String;
    fn magenta(self) -> String;
    fn cyan(self) -> String;
    fn light_grey(self) -> String;
    fn grey(self) -> String;
    fn light_red(self) -> String;
    fn light_green(self) -> String;
    fn light_yellow(self) -> String;
    fn light_blue(self) -> String;
    fn light_magenta(self) -> String;
    fn light_cyan(self) -> String;
    fn white(self) -> String;
    fn strong(self) -> String;

    fn bold(self) -> String;
    fn italic(self) -> String;
    fn underline(self) -> String;
    fn blink(self) -> String;
    fn reverse(self) -> String;
    fn hidden(self) -> String;
    fn crossed(self) -> String;

    fn close(self) -> String;
}

impl Colored for String {
    fn close(self) -> String {
        format!("{}{}", self, Style::None)
    }
    fn bg_black(self) -> String {
        format!("{}{}", Background::Black, self)
    }
    fn bg_red(self) -> String {
        format!("{}{}", Background::Red, self)
    }
    fn bg_green(self) -> String {
        format!("{}{}", Background::Green, self)
    }
    fn bg_yellow(self) -> String {
        format!("{}{}", Background::Yellow, self)
    }
    fn bg_blue(self) -> String {
        format!("{}{}", Background::Blue, self)
    }
    fn bg_magenta(self) -> String {
        format!("{}{}", Background::Magenta, self)
    }
    fn bg_cyan(self) -> String {
        format!("{}{}", Background::Cyan, self)
    }
    fn bg_light_grey(self) -> String {
        format!("{}{}", Background::LightGrey, self)
    }
    fn bg_grey(self) -> String {
        format!("{}{}", Background::Grey, self)
    }
    fn bg_light_red(self) -> String {
        format!("{}{}", Background::LightRed, self)
    }
    fn bg_light_green(self) -> String {
        format!("{}{}", Background::LightGreen, self)
    }
    fn bg_light_yellow(self) -> String {
        format!("{}{}", Background::LightYellow, self)
    }
    fn bg_light_blue(self) -> String {
        format!("{}{}", Background::LightBlue, self)
    }
    fn bg_light_magenta(self) -> String {
        format!("{}{}", Background::LightMagenta, self)
    }
    fn bg_light_cyan(self) -> String {
        format!("{}{}", Background::LightCyan, self)
    }
    fn bg_white(self) -> String {
        format!("{}{}", Background::White, self)
    }

    fn fg_black(self) -> String {
        format!("{}{}", Foreground::Black, self)
    }
    fn red(self) -> String {
        format!("{}{}", Foreground::Red, self)
    }
    fn green(self) -> String {
        format!("{}{}", Foreground::Green, self)
    }
    fn yellow(self) -> String {
        format!("{}{}", Foreground::Yellow, self)
    }
    fn blue(self) -> String {
        format!("{}{}", Foreground::Blue, self)
    }
    fn magenta(self) -> String {
        format!("{}{}", Foreground::Magenta, self)
    }
    fn cyan(self) -> String {
        format!("{}{}", Foreground::Cyan, self)
    }
    fn light_grey(self) -> String {
        format!("{}{}", Foreground::LightGrey, self)
    }
    fn grey(self) -> String {
        format!("{}{}", Foreground::Grey, self)
    }
    fn light_red(self) -> String {
        format!("{}{}", Foreground::LightRed, self)
    }
    fn light_green(self) -> String {
        format!("{}{}", Foreground::LightGreen, self)
    }
    fn light_yellow(self) -> String {
        format!("{}{}", Foreground::LightYellow, self)
    }
    fn light_blue(self) -> String {
        format!("{}{}", Foreground::LightBlue, self)
    }
    fn light_magenta(self) -> String {
        format!("{}{}", Foreground::LightMagenta, self)
    }

    fn light_cyan(self) -> String {
        format!("{}{}", Foreground::LightCyan, self)
    }
    fn white(self) -> String {
        format!("{}{}", Foreground::White, self)
    }
    fn strong(self) -> String {
        format!("{}{}", Foreground::Strong, self)
    }

    fn bold(self) -> String {
        format!("{}{}", Style::Bold, self)
    }
    fn italic(self) -> String {
        format!("{}{}", Style::Italic, self)
    }
    fn underline(self) -> String {
        format!("{}{}", Style::Underline, self)
    }
    fn blink(self) -> String {
        format!("{}{}", Style::Blink, self)
    }
    fn reverse(self) -> String {
        format!("{}{}", Style::Reverse, self)
    }
    fn hidden(self) -> String {
        format!("{}{}", Style::Hidden, self)
    }
    fn crossed(self) -> String {
        format!("{}{}", Style::Crossed, self)
    }
}

impl Colored for &str {
    fn close(self) -> String {
        format!("{}{}", self, Style::None)
    }
    fn bg_black(self) -> String {
        format!("{}{}", Background::Black, self)
    }
    fn bg_red(self) -> String {
        format!("{}{}", Background::Red, self)
    }
    fn bg_green(self) -> String {
        format!("{}{}", Background::Green, self)
    }
    fn bg_yellow(self) -> String {
        format!("{}{}", Background::Yellow, self)
    }
    fn bg_blue(self) -> String {
        format!("{}{}", Background::Blue, self)
    }
    fn bg_magenta(self) -> String {
        format!("{}{}", Background::Magenta, self)
    }
    fn bg_cyan(self) -> String {
        format!("{}{}", Background::Cyan, self)
    }
    fn bg_light_grey(self) -> String {
        format!("{}{}", Background::LightGrey, self)
    }
    fn bg_grey(self) -> String {
        format!("{}{}", Background::Grey, self)
    }
    fn bg_light_red(self) -> String {
        format!("{}{}", Background::LightRed, self)
    }
    fn bg_light_green(self) -> String {
        format!("{}{}", Background::LightGreen, self)
    }
    fn bg_light_yellow(self) -> String {
        format!("{}{}", Background::LightYellow, self)
    }
    fn bg_light_blue(self) -> String {
        format!("{}{}", Background::LightBlue, self)
    }
    fn bg_light_magenta(self) -> String {
        format!("{}{}", Background::LightMagenta, self)
    }
    fn bg_light_cyan(self) -> String {
        format!("{}{}", Background::LightCyan, self)
    }
    fn bg_white(self) -> String {
        format!("{}{}", Background::White, self)
    }

    fn fg_black(self) -> String {
        format!("{}{}", Foreground::Black, self)
    }
    fn red(self) -> String {
        format!("{}{}", Foreground::Red, self)
    }
    fn green(self) -> String {
        format!("{}{}", Foreground::Green, self)
    }
    fn yellow(self) -> String {
        format!("{}{}", Foreground::Yellow, self)
    }
    fn blue(self) -> String {
        format!("{}{}", Foreground::Blue, self)
    }
    fn magenta(self) -> String {
        format!("{}{}", Foreground::Magenta, self)
    }
    fn cyan(self) -> String {
        format!("{}{}", Foreground::Cyan, self)
    }
    fn light_grey(self) -> String {
        format!("{}{}", Foreground::LightGrey, self)
    }
    fn grey(self) -> String {
        format!("{}{}", Foreground::Grey, self)
    }
    fn light_red(self) -> String {
        format!("{}{}", Foreground::LightRed, self)
    }
    fn light_green(self) -> String {
        format!("{}{}", Foreground::LightGreen, self)
    }
    fn light_yellow(self) -> String {
        format!("{}{}", Foreground::LightYellow, self)
    }
    fn light_blue(self) -> String {
        format!("{}{}", Foreground::LightBlue, self)
    }
    fn light_magenta(self) -> String {
        format!("{}{}", Foreground::LightMagenta, self)
    }

    fn light_cyan(self) -> String {
        format!("{}{}", Foreground::LightCyan, self)
    }
    fn white(self) -> String {
        format!("{}{}", Foreground::White, self)
    }
    fn strong(self) -> String {
        format!("{}{}", Foreground::Strong, self)
    }

    fn bold(self) -> String {
        format!("{}{}", Style::Bold, self)
    }
    fn italic(self) -> String {
        format!("{}{}", Style::Italic, self)
    }
    fn underline(self) -> String {
        format!("{}{}", Style::Underline, self)
    }
    fn blink(self) -> String {
        format!("{}{}", Style::Blink, self)
    }
    fn reverse(self) -> String {
        format!("{}{}", Style::Reverse, self)
    }
    fn hidden(self) -> String {
        format!("{}{}", Style::Hidden, self)
    }
    fn crossed(self) -> String {
        format!("{}{}", Style::Crossed, self)
    }
}
