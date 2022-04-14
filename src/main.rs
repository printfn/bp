use std::{
    error,
    io::{self, Write},
    process,
};

pub type Error = Box<dyn error::Error + Send + Sync + 'static>;

use copypasta::{ClipboardContext, ClipboardProvider};

fn stdin_tty() -> bool {
    atty::is(atty::Stream::Stdin)
}

fn stdout_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

fn copy(mut file: impl io::Read) -> Result<Vec<u8>, Error> {
    let mut ctx = ClipboardContext::new()?;
    let mut contents = Vec::with_capacity(32);
    file.read_to_end(&mut contents)?;
    ctx.set_contents(String::from_utf8(contents.clone())?)?;
    Ok(contents)
}

fn paste() -> Result<String, Error> {
    let mut ctx = ClipboardContext::new()?;
    let contents = ctx.get_contents()?;
    Ok(contents)
}

fn inner_main() -> Result<(), Error> {
    match (stdin_tty(), stdout_tty()) {
        (true, true) => {
            let contents = paste()?;
            print!("{}", contents);
            if !contents.ends_with('\n') {
                println!();
            }
        }
        (true, false) => {
            print!("{}", paste()?);
        }
        (false, true) => {
            copy(io::stdin())?;
        }
        (false, false) => {
            let contents = copy(io::stdin())?;
            io::stdout().write_all(&contents)?;
        }
    }
    Ok(())
}

fn main() {
    match inner_main() {
        Ok(()) => (),
        Err(e) => {
            eprint!("{}", e);
            while let Some(e) = e.source() {
                eprint!(": {}", e);
            }
            eprintln!();
            process::exit(1);
        }
    }
}
