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