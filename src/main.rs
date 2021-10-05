use tempera::*;

fn main() {
  println!("{}", colorize("Colorized", &["red"]));
  println!("{}", colorize("Colorized", &["red bgBlue"]));
  println!("{}", colorize("Colorized", &["ansi:100"]));
  println!("{}", colorize("Colorized", &["bgANSI:3,4,5"]));
  println!("{}", colorize("Colorized", &["rgb:255,0,0"]));
  println!("{}", colorize("Colorized", &["bgRGB:0,255,0"]));
  println!("{}", colorize("Colorized", &["hex:#FF0000"]));
  println!("{}", colorize("Colorized", &["bgHEX:00FF00"]));
  println!(
    "{}",
    colorize_template("{red}This is in red, {green underline}this in green underlined{-}, this in red again.")
  );
}
