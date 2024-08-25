# cli-colors

cli-colors is a Rust library that provides a simple way to add ANSI color and formatting to your command-line interface (CLI) applications. With cli-color, you can easily add underlines, bold text, italics, and various colors to your CLI output.

## Features

- Easy-to-use API for adding color and formatting to CLI text
- Support for common text styles:
  - Bold
  - Italic
  - Underline
- Wide range of color options for both foreground and background
- Combine multiple styles and colors

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cli-colors = "1.0.0"
```

## Usage

Here's a quick example of how to use cli-colors:

```rs
use cli_colors::Colorizer;

fn main() {
    let colorizer = Colorizer::new();
    
    println!("{}", colorizer.black("Black"));
    println!("{}", colorizer.blue("Blue"));
    println!("{}", colorizer.bold("Bold"));
    println!("{}", colorizer.bright_black("Bright Black"));
    println!("{}", colorizer.bright_red("Bright Red"));
    println!("{}", colorizer.cyan("Cyan"));
    println!("{}", colorizer.green("Green"));
    println!("{}", colorizer.italic("Italic"));
    println!("{}", colorizer.magenta("Magenta"));
    println!("{}", colorizer.red("Red"));
    println!("{}", colorizer.strikethrough("Strikethrough"));
    println!("{}", colorizer.underline("Underline"));
    println!("{}", colorizer.white("White"));
    println!("{}", colorizer.yellow("Yellow"));
    println!("{}", colorizer.rgb("Custom", 255, 0, 255));
    println!("{}", colorizer.rainbow("THIS IS A PRETTY RAINBOW WOOT WOOT"));
    println!("{}", colorizer.bg_rainbow("THIS IS A PRETTY RAINBOW WOOT WOOT"));
}
```

## License

This project is licensed under the MIT 