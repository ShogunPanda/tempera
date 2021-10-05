# tempera

Template based terminal coloring made really easy.

## Usage

Tempera allows to add coloring to terminal in a really easy way.

### Colorize strings

To colorize strings, simply use the [colorize]() function, passing a vector of styles you want to apply.
The list of supported color names correspondes to the keys of [CODES]() variable.

```rust
use tempera::*;

fn main() {
  println!("{}", colorize("Colorized", &["red"]));
  println!("{}", colorize("Colorized", &["red bgBlue"]));
}
```

### Colorize templates

To colorize a template using a tagged template syntax, simply use the [colorize_template]() function.

```rust
use tempera::*;

fn main() {
  println!("{}", colorize_template("{red}This is in red, {green underline}this in green underlined{-}, this in red again.");
}
```

The template recognizes styles between curly braces (use a single or double opening brace to escape them) and the token `{-}` as universal closing tag (which also restores the previous style).

The closing tag at the end of the string can be omitted, since tempera will append the global reset style (`\u{1b}[0m`) if any style was set.

If you want to discard styles to be restored, use the `{reset}` token.

### Setting custom styles

If you want to define custom styles, use the [add_style]() function.

```rust
use tempera::*;

fn main() {
  println!("{}", add_style("custom", &["yellow", "bgRed"]).is_ok());
}
```

### 256 and 16 millions colors support

tempera supports 256 ANSI codes and 16m RGB colors. Just give it a try:

```rust
use tempera::*;

fn main() {
  println!("{}", colorize("Colorized", &["ansi:100"]));
  println!("{}", colorize("Colorized", &["bgANSI:3,4,5"]));
  println!("{}", colorize("Colorized", &["rgb:255,0,0"]));
  println!("{}", colorize("Colorized", &["bgRGB:0,255,0"]));
  println!("{}", colorize("Colorized", &["hex:#FF0000"]));
  println!("{}", colorize("Colorized", &["bgHEX:00FF00"]));
}
```

ANSI, RGB, and HEX can be used in style definitions and templates as well.

## API Documentation

The API documentation can be found [here]().

## Contributing to tempera

- Check out the latest master to make sure the feature hasn't been implemented or the bug hasn't been fixed yet.
- Check out the issue tracker to make sure someone already hasn't requested it and/or contributed it.
- Fork the project.
- Start a feature/bugfix branch.
- Commit and push until you are happy with your contribution.
- Make sure to add tests for it. This is important so I don't break it in a future version unintentionally.

## Copyright

Copyright (C) 2021 and above Shogun (shogun@cowtech.it).

Licensed under the ISC license, which can be found at https://choosealicense.com/licenses/isc.
