use std::io::{Read, Write};

use copypasta::{ClipboardContext, ClipboardProvider};

// based on https://gist.github.com/RichardBronosky/56d8f614fab2bacdd8b048fb58d0c0c7

fn stdin_tty() -> bool {
    atty::is(atty::Stream::Stdin)
}

fn stdout_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

fn paste() {
    let mut ctx = ClipboardContext::new().unwrap();
    let contents = ctx.get_contents().unwrap();
    std::io::stdout().write(contents.as_bytes()).unwrap();
    if !contents.ends_with('\n') && stdout_tty() {
        println!();
    }
}

fn copy() {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut string = String::new();
    std::io::stdin().read_to_string(&mut string).unwrap();
    ctx.set_contents(string).unwrap();
}

fn main() {
    if !stdin_tty() {
        copy()
    }
    paste()
}
