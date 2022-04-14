use super::Error;
use std::io;

use copypasta::{ClipboardContext, ClipboardProvider};

pub fn copy(mut file: impl io::Read) -> Result<Vec<u8>, Error> {
    let mut ctx = ClipboardContext::new()?;
    let mut contents = Vec::with_capacity(32);
    file.read_to_end(&mut contents)?;
    ctx.set_contents(String::from_utf8(contents.clone())?)?;
    Ok(contents)
}

pub fn paste() -> Result<Vec<u8>, Error> {
    let mut ctx = ClipboardContext::new()?;
    let contents = ctx.get_contents()?.into_bytes();
    Ok(contents)
}
