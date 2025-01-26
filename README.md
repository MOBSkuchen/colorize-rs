colorize-rs
========
A fork of colorize https://github.com/jeremyletang/colorize

``cargo add colorize-rs``

__colorize-rs__ provide simple text colorization for terminal emulator, using ansi escape characters.

Look at this example:

```Rust
extern crate colorize_rs;
use colorize_rs::{AnsiColor, Color};

pub fn main() {
    // Set some global colors
    colorize_rs::global_fg(Color::Blue);
    colorize_rs::global_bg(Color::Red);
    // ^~~~ These settings are reset to default at the end.

    // You can use specific colors or style on a given str,
    // the globals colors are restored after !

    // Write a green underlined text on a yellow background !
    println!("{}", "Hello World !".green().underlined().yellowb());

    // Use bright or normal colors
    println!("{}", "Bright Green foreground and Magenta background !".b_green().magentab());
}
```
