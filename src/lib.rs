/*
Terminal color using ansi escape character for Rust.
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
*/

use BgColor::*;
use Color::*;
use Style::*;

use std::mem;

/// Ansi color to set the global foreground / background color
#[derive(Clone, Copy)]
pub enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    Grey = 37,
    Default = 39,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightGrey = 97,
}

#[derive(Clone, Copy)]
pub enum BgColor {
    Blackb = 40,
    Redb = 41,
    Greenb = 42,
    Yellowb = 43,
    Blueb = 44,
    Magentab = 45,
    Cyanb = 46,
    Greyb = 47,
    Defaultb = 49,
    BrightBlackb = 100,
    BrightRedb = 101,
    BrightGreenb = 102,
    BrightYellowb = 103,
    BrightBlueb = 104,
    BrightMagentab = 105,
    BrightCyanb = 106,
    BrightGreyb = 107,
}

#[derive(Clone, Copy)]
pub enum Style {
    Underscore = 4,
    Bold = 1,
    Blink = 5,
    Reverse = 7,
    Concealed = 8,
    Faint = 2,
    Italic = 3,
    CrossedOut = 9,
}

impl internal::TermAttrib for Color {
    fn to_int(&self) -> i32 {
        *self as i32
    }
}

impl internal::TermAttrib for BgColor {
    fn to_int(&self) -> i32 {
        *self as i32
    }
}

impl internal::TermAttrib for Style {
    fn to_int(&self) -> i32 {
        *self as i32
    }
}

impl BgColor {
    fn from_fg(color: Color) -> BgColor {
        unsafe { mem::transmute(color as i8 + 10) }
    }
}

mod internal {
    use super::{BgColor, Color};
    use std::cell::RefCell;

    static DEFAULT_FG: i32 = 39;
    static DEFAULT_BG: i32 = 49;
    thread_local!(static GLOB_COLOR: RefCell<GlobalColor> = RefCell::new(GlobalColor {fg: DEFAULT_FG, bg: DEFAULT_BG, enabled: true}));

    pub trait TermAttrib {
        fn to_int(&self) -> i32;
    }

    #[derive(Clone)]
    pub struct GlobalColor {
        fg: i32,
        bg: i32,
        enabled: bool,
    }

    impl Drop for GlobalColor {
        fn drop(&mut self) {
            print!("\x1b[0;{};{}m", DEFAULT_FG, DEFAULT_BG)
        }
    }

    fn get_glob() -> (i32, i32, bool) {
        GLOB_COLOR.with(|cell| {
            let g = cell.borrow();
            (g.fg, g.bg, g.enabled)
        })
    }

    pub fn global_color(fg_color: Option<Color>, bg_color: Option<BgColor>, enabled: Option<bool>) {
        GLOB_COLOR.with(|cell| {
            let mut g = cell.borrow_mut();
            match fg_color {
                Some(c) => g.fg = c.to_int(),
                None => g.fg = g.fg,
            }
            match bg_color {
                Some(c) => g.bg = c.to_int(),
                None => g.bg = g.bg,
            }
            match enabled {
                Some(c) => g.enabled = c,
                None => g.enabled = g.enabled,
            }
        })
    }

    pub fn pack<T: TermAttrib>(attrib: T, mut text: String) -> String {
        if {
            let (_, _, en) = get_glob();
            !en
        } {
            return text;
        }
        if text.as_str().starts_with("\x1b[") {
            unsafe {
                text.as_mut_vec().remove(0);
                text.as_mut_vec().remove(0);
            }
            let tmp = text;
            text = String::from("\x1b[");
            text.push_str(format!("{};", attrib.to_int()).as_str());
            text.push_str(tmp.as_str());
        } else {
            let tmp = text;
            text = format!("\x1b[{}m", attrib.to_int());
            text.push_str(tmp.as_str());
            let (fg, bg, _) = get_glob();
            text.push_str(format!("\x1b[0;{};{}m", fg, bg).as_str());
        }
        text
    }
}

/// Set a custom global foreground color
pub fn global_fg(color: Color) {
    internal::global_color(Some(color), None, None)
}

/// Set a custom global background color
pub fn global_bg(color: Color) {
    internal::global_color(None, Some(BgColor::from_fg(color)), None)
}
/// Set color to be enabled / disabled
pub fn color_enabled(enabled: bool) {
    internal::global_color(None, None, Some(enabled))
}

/// Reset the background and foreground color to the defaults colors
pub fn reset() {
    internal::global_color(Some(Default), Some(Defaultb), None)
}

/// Methods extension to colorize the text contained in a string
/// using a simple mathod call
pub trait AnsiColor {
    /// Foreground black
    fn black(self) -> String;
    /// Foreground red
    fn red(self) -> String;
    /// Foreground green
    fn green(self) -> String;
    /// Foreground yellow
    fn yellow(self) -> String;
    /// Foreground blue
    fn blue(self) -> String;
    /// Foreground magenta
    fn magenta(self) -> String;
    /// Foreground cyan
    fn cyan(self) -> String;
    /// Foreground grey
    fn grey(self) -> String;
    /// Foreground black bright
    fn b_black(self) -> String;
    /// Foreground red bright
    fn b_red(self) -> String;
    /// Foreground green bright
    fn b_green(self) -> String;
    /// Foreground yellow bright
    fn b_yellow(self) -> String;
    /// Foreground blue bright
    fn b_blue(self) -> String;
    /// Foreground magenta bright
    fn b_magenta(self) -> String;
    /// Foreground cyan bright
    fn b_cyan(self) -> String;
    /// Foreground grey bright
    fn b_grey(self) -> String;
    /// Foreground default
    fn default(self) -> String;

    /// Background black
    fn blackb(self) -> String;
    /// Background red
    fn redb(self) -> String;
    /// Background green
    fn greenb(self) -> String;
    /// Background yellow
    fn yellowb(self) -> String;
    /// Background bblue
    fn blueb(self) -> String;
    /// Background magenta
    fn magentab(self) -> String;
    /// Background cyan
    fn cyanb(self) -> String;
    /// Background grey
    fn greyb(self) -> String;
    /// Background black bright
    fn b_blackb(self) -> String;
    /// Background red bright
    fn b_redb(self) -> String;
    /// Background green bright
    fn b_greenb(self) -> String;
    /// Background yellow bright
    fn b_yellowb(self) -> String;
    /// Background bblue bright
    fn b_blueb(self) -> String;
    /// Background magenta bright
    fn b_magentab(self) -> String;
    /// Background cyan bright
    fn b_cyanb(self) -> String;
    /// Background grey bright
    fn b_greyb(self) -> String;
    /// Background default
    fn defaultb(self) -> String;

    /// Text underlined
    fn underlined(self) -> String;
    /// Bold text
    fn bold(self) -> String;
    /// Blink tests ( Wonderful )
    fn blink(self) -> String;
    /// Reverse mod ON
    fn reverse(self) -> String;
    /// Concealed mod ON
    fn concealed(self) -> String;
    /// Faint mod ON
    fn faint(self) -> String;
    /// Italic text
    fn italic(self) -> String;
    /// Crossed out
    fn crossedout(self) -> String;
}

impl AnsiColor for String {
    // Foreground
    fn black(self) -> String {
        internal::pack(Black, self)
    }
    fn red(self) -> String {
        internal::pack(Red, self)
    }
    fn green(self) -> String {
        internal::pack(Green, self)
    }
    fn yellow(self) -> String {
        internal::pack(Yellow, self)
    }
    fn blue(self) -> String {
        internal::pack(Blue, self)
    }
    fn magenta(self) -> String {
        internal::pack(Magenta, self)
    }
    fn cyan(self) -> String {
        internal::pack(Cyan, self)
    }
    fn grey(self) -> String {
        internal::pack(Grey, self)
    }
    fn b_black(self) -> String {
        internal::pack(BrightBlack, self)
    }
    fn b_red(self) -> String {
        internal::pack(BrightRed, self)
    }
    fn b_green(self) -> String {
        internal::pack(BrightGreen, self)
    }
    fn b_yellow(self) -> String {
        internal::pack(BrightYellow, self)
    }
    fn b_blue(self) -> String {
        internal::pack(BrightBlue, self)
    }
    fn b_magenta(self) -> String {
        internal::pack(BrightMagenta, self)
    }
    fn b_cyan(self) -> String {
        internal::pack(BrightCyan, self)
    }
    fn b_grey(self) -> String {
        internal::pack(BrightGrey, self)
    }
    fn default(self) -> String {
        internal::pack(Default, self)
    }

    // Background
    fn blackb(self) -> String {
        internal::pack(Blackb, self)
    }
    fn redb(self) -> String {
        internal::pack(Redb, self)
    }
    fn greenb(self) -> String {
        internal::pack(Greenb, self)
    }
    fn yellowb(self) -> String {
        internal::pack(Yellowb, self)
    }
    fn blueb(self) -> String {
        internal::pack(Blueb, self)
    }
    fn magentab(self) -> String {
        internal::pack(Magentab, self)
    }
    fn cyanb(self) -> String {
        internal::pack(Cyanb, self)
    }
    fn greyb(self) -> String {
        internal::pack(Greyb, self)
    }
    fn b_blackb(self) -> String {
        internal::pack(BrightBlackb, self)
    }
    fn b_redb(self) -> String {
        internal::pack(BrightRedb, self)
    }
    fn b_greenb(self) -> String {
        internal::pack(BrightGreenb, self)
    }
    fn b_yellowb(self) -> String {
        internal::pack(BrightYellowb, self)
    }
    fn b_blueb(self) -> String {
        internal::pack(BrightBlueb, self)
    }
    fn b_magentab(self) -> String {
        internal::pack(BrightMagentab, self)
    }
    fn b_cyanb(self) -> String {
        internal::pack(BrightCyanb, self)
    }
    fn b_greyb(self) -> String {
        internal::pack(BrightGreyb, self)
    }
    fn defaultb(self) -> String {
        internal::pack(Defaultb, self)
    }

    // styles
    fn underlined(self) -> String {
        internal::pack(Underscore, self)
    }
    fn bold(self) -> String {
        internal::pack(Bold, self)
    }
    fn blink(self) -> String {
        internal::pack(Blink, self)
    }
    fn reverse(self) -> String {
        internal::pack(Reverse, self)
    }
    fn concealed(self) -> String {
        internal::pack(Concealed, self)
    }
    fn faint(self) -> String {
        internal::pack(Faint, self)
    }
    fn italic(self) -> String {
        internal::pack(Italic, self)
    }
    fn crossedout(self) -> String {
        internal::pack(CrossedOut, self)
    }
}

impl AnsiColor for &'static str {
    // Foreground
    fn black(self) -> String {
        String::from(self).black()
    }
    fn red(self) -> String {
        String::from(self).red()
    }
    fn green(self) -> String {
        String::from(self).green()
    }
    fn yellow(self) -> String {
        String::from(self).yellow()
    }
    fn blue(self) -> String {
        String::from(self).blue()
    }
    fn magenta(self) -> String {
        String::from(self).magenta()
    }
    fn cyan(self) -> String {
        String::from(self).cyan()
    }
    fn grey(self) -> String {
        String::from(self).grey()
    }
    fn b_black(self) -> String {
        String::from(self).b_black()
    }
    fn b_red(self) -> String {
        String::from(self).b_red()
    }
    fn b_green(self) -> String {
        String::from(self).b_green()
    }
    fn b_yellow(self) -> String {
        String::from(self).b_yellow()
    }
    fn b_blue(self) -> String {
        String::from(self).b_blue()
    }
    fn b_magenta(self) -> String {
        String::from(self).b_magenta()
    }
    fn b_cyan(self) -> String {
        String::from(self).b_cyan()
    }
    fn b_grey(self) -> String {
        String::from(self).b_grey()
    }
    fn default(self) -> String {
        String::from(self).default()
    }

    // Background
    fn blackb(self) -> String {
        String::from(self).blackb()
    }
    fn redb(self) -> String {
        String::from(self).redb()
    }
    fn greenb(self) -> String {
        String::from(self).greenb()
    }
    fn yellowb(self) -> String {
        String::from(self).yellowb()
    }
    fn blueb(self) -> String {
        String::from(self).blueb()
    }
    fn magentab(self) -> String {
        String::from(self).magentab()
    }
    fn cyanb(self) -> String {
        String::from(self).cyanb()
    }
    fn greyb(self) -> String {
        String::from(self).greyb()
    }
    fn b_blackb(self) -> String {
        String::from(self).b_blackb()
    }
    fn b_redb(self) -> String {
        String::from(self).b_redb()
    }
    fn b_greenb(self) -> String {
        String::from(self).b_greenb()
    }
    fn b_yellowb(self) -> String {
        String::from(self).b_yellowb()
    }
    fn b_blueb(self) -> String {
        String::from(self).b_blueb()
    }
    fn b_magentab(self) -> String {
        String::from(self).b_magentab()
    }
    fn b_cyanb(self) -> String {
        String::from(self).b_cyanb()
    }
    fn b_greyb(self) -> String {
        String::from(self).b_greyb()
    }
    fn defaultb(self) -> String {
        String::from(self).defaultb()
    }

    // styles
    fn underlined(self) -> String {
        String::from(self).underlined()
    }
    fn bold(self) -> String {
        String::from(self).bold()
    }
    fn blink(self) -> String {
        String::from(self).blink()
    }
    fn reverse(self) -> String {
        String::from(self).reverse()
    }
    fn concealed(self) -> String {
        String::from(self).concealed()
    }
    fn faint(self) -> String {
        String::from(self).faint()
    }
    fn italic(self) -> String {
        String::from(self).italic()
    }
    fn crossedout(self) -> String {
        String::from(self).crossedout()
    }
}
