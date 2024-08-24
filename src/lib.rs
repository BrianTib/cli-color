use std::fmt::Display;

pub enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    // Bright colors (requires aixterm)
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

pub struct Colorizer;

impl Colorizer {
    pub fn new() -> Colorizer {
        Colorizer
    }

    /// This is needed to determine if features such as bright colors are supported.
    fn supports_aixterm() -> bool {
        std::env::var("TERM").unwrap_or_default().contains("xterm")
    }

    fn ansi<T: Display>(&self, input: T, ansi_code: u8) -> String {
        format!("\x1b[{}m{}\x1b[0m", ansi_code, input)
    }

    pub fn underline<T: Display>(&self, input: T) -> String {
        self.ansi(input, 4)
    }

    pub fn bold<T: Display>(&self, input: T) -> String {
        self.ansi(input, 1)
    }

    pub fn italic<T: Display>(&self, input: T) -> String {
        self.ansi(input, 3)
    }

    pub fn strikethrough<T: Display>(&self, input: T) -> String {
        self.ansi(input, 9)
    }

    pub fn black<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::Black as u8)
    }

    pub fn red<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::Red as u8)
    }

    pub fn green<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::Green as u8)
    }

    pub fn yellow<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::Yellow as u8)
    }

    pub fn blue<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::Blue as u8)
    }

    pub fn magenta<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::Magenta as u8)
    }

    pub fn cyan<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::Cyan as u8)
    }

    pub fn white<T: Display>(&self, input: T) -> String {
        self.ansi(input, Color::White as u8)
    }

    pub fn bright_black<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightBlack as u8)
    }

    pub fn bright_red<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightRed as u8)
    }

    pub fn bright_green<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightGreen as u8)
    }

    pub fn bright_yellow<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightYellow as u8)
    }

    pub fn bright_blue<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightBlue as u8)
    }

    pub fn bright_magenta<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightMagenta as u8)
    }

    pub fn bright_cyan<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightCyan as u8)
    }

    pub fn bright_white<T: Display>(&self, input: T) -> String {
        assert!(
            Self::supports_aixterm(),
            "Bright colors require aixterm support",
        );
        self.ansi(input, Color::BrightWhite as u8)
    }

    pub fn bg_black<T: Display>(&self, input: T) -> String {
        format!("\x1b[40m{}\x1b[0m", input)
    }

    pub fn bg_red<T: Display>(&self, input: T) -> String {
        format!("\x1b[41m{}\x1b[0m", input)
    }

    pub fn bg_green<T: Display>(&self, input: T) -> String {
        format!("\x1b[42m{}\x1b[0m", input)
    }

    pub fn bg_yellow<T: Display>(&self, input: T) -> String {
        format!("\x1b[43m{}\x1b[0m", input)
    }

    pub fn bg_blue<T: Display>(&self, input: T) -> String {
        format!("\x1b[44m{}\x1b[0m", input)
    }

    pub fn bg_magenta<T: Display>(&self, input: T) -> String {
        format!("\x1b[45m{}\x1b[0m", input)
    }

    pub fn bg_cyan<T: Display>(&self, input: T) -> String {
        format!("\x1b[46m{}\x1b[0m", input)
    }

    pub fn bg_white<T: Display>(&self, input: T) -> String {
        format!("\x1b[47m{}\x1b[0m", input)
    }

    pub fn rgb<T: Display>(&self, input: T, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, input)
    }

    pub fn bg_rgb<T: Display>(&self, input: T, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, input)
    }

    pub fn rgb256<T: Display>(&self, input: T, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;5;{}m{}\x1b[0m", 16 + 36 * r + 6 * g + b, input)
    }

    pub fn bg_rgb256<T: Display>(&self, input: T, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;5;{}m{}\x1b[0m", 16 + 36 * r + 6 * g + b, input)
    }

    pub fn reset<T: Display>(&self, input: T) -> String {
        format!("\x1b[0m{}\x1b[0m", input)
    }

    pub fn bold_underline<T: Display>(&self, input: T) -> String {
        self.ansi(self.underline(self.bold(input)), 1)
    }

    pub fn bold_italic<T: Display>(&self, input: T) -> String {
        self.ansi(self.italic(self.bold(input)), 1)
    }

    pub fn bold_strikethrough<T: Display>(&self, input: T) -> String {
        self.ansi(self.strikethrough(self.bold(input)), 1)
    }

    pub fn underline_italic<T: Display>(&self, input: T) -> String {
        self.ansi(self.italic(self.underline(input)), 1)
    }

    pub fn underline_strikethrough<T: Display>(&self, input: T) -> String {
        self.ansi(self.strikethrough(self.underline(input)), 1)
    }

    pub fn italic_strikethrough<T: Display>(&self, input: T) -> String {
        self.ansi(self.strikethrough(self.italic(input)), 1)
    }

    pub fn bold_underline_italic<T: Display>(&self, input: T) -> String {
        self.ansi(self.italic(self.underline(self.bold(input))), 1)
    }

    pub fn bold_underline_strikethrough<T: Display>(&self, input: T) -> String {
        self.ansi(self.strikethrough(self.underline(self.bold(input))), 1)
    }

    pub fn bold_italic_strikethrough<T: Display>(&self, input: T) -> String {
        self.ansi(self.strikethrough(self.italic(self.bold(input))), 1)
    }

    pub fn underline_italic_strikethrough<T: Display>(&self, input: T) -> String {
        self.ansi(self.strikethrough(self.italic(self.underline(input))), 1)
    }

    pub fn rainbow<T: Display>(&self, input: T) -> String {
        let mut rainbow = String::new();
        for (i, c) in input.to_string().chars().enumerate() {
            let color = match i % 6 {
                0 => self.red(c),
                1 => self.yellow(c),
                2 => self.green(c),
                3 => self.cyan(c),
                4 => self.blue(c),
                5 => self.magenta(c),
                _ => unreachable!(),
            };
            rainbow.push_str(&color);
        }
        rainbow
    }

    pub fn bg_rainbow<T: Display>(&self, input: T) -> String {
        let mut rainbow = String::new();
        for (i, c) in input.to_string().chars().enumerate() {
            let color = match i % 6 {
                0 => self.bg_red(c),
                1 => self.bg_yellow(c),
                2 => self.bg_green(c),
                3 => self.bg_cyan(c),
                4 => self.bg_blue(c),
                5 => self.bg_magenta(c),
                _ => unreachable!(),
            };
            rainbow.push_str(&color);
        }
        rainbow
    }

    pub fn bg_rainbow256<T: Display>(&self, input: T) -> String {
        let mut rainbow = String::new();
        for (i, c) in input.to_string().chars().enumerate() {
            let color = match i % 6 {
                0 => self.bg_rgb256(c, 5, 0, 0),
                1 => self.bg_rgb256(c, 5, 5, 0),
                2 => self.bg_rgb256(c, 0, 5, 0),
                3 => self.bg_rgb256(c, 0, 5, 5),
                4 => self.bg_rgb256(c, 0, 0, 5),
                5 => self.bg_rgb256(c, 5, 0, 5),
                _ => unreachable!(),
            };
            rainbow.push_str(&color);
        }
        rainbow
    }

    pub fn bg_rainbow_rgb<T: Display>(&self, input: T) -> String {
        let mut rainbow = String::new();
        for (i, c) in input.to_string().chars().enumerate() {
            let color = match i % 6 {
                0 => self.bg_rgb(c, 255, 0, 0),
                1 => self.bg_rgb(c, 255, 255, 0),
                2 => self.bg_rgb(c, 0, 255, 0),
                3 => self.bg_rgb(c, 0, 255, 255),
                4 => self.bg_rgb(c, 0, 0, 255),
                5 => self.bg_rgb(c, 255, 0, 255),
                _ => unreachable!(),
            };
            rainbow.push_str(&color);
        }
        rainbow
    }
}